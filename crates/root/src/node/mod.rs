use crate::core::NsvCore;
use crate::util::create_node_version_vaildate_reg;
use crate::util::dir::ensure_dir;
use crate::util::download::{download_file, unzip_file};
use async_trait::async_trait;
use serde::Deserialize;
use tokio::fs::rename;
use std::fs::{read_dir, DirEntry};
use std::path::{Path, PathBuf};

#[derive(PartialEq, Debug, Clone)]
pub enum VersionTarget {
    Lts,
    Latest,
    Assign(String),
}

#[derive(PartialEq, Debug)]
pub enum NsvCoreError {
    /**
     * 自定义报错信息
     */
    String(String),

    /**
     * 空值
     */
    Empty,

    /**
     * 非法版本
     */
    IllegalityVersion(String),

    /**
     * node 版本远程不存在
     */
    NodeVersionRemoteNotFound,

    /**
     * node版本本地不存在
     */
    NodeVersionLocalNotFound,

    /**
     * 本地已存在
     */
    NodeVersionLocalExist,
}

#[async_trait]
pub trait NodeDispose {
    /// 根据版本str获取本地 node  dir
    fn get_local_node_dir_2_dir_entry(&self, version: &str) -> Option<DirEntry>;

    /// 格式化用户输入的 版本
    fn set_version_target(&mut self, version: &str) -> Result<(), NsvCoreError>;

    /// 格式化node版本
    fn format_version(&self, version: &str) -> Result<String, NsvCoreError>;

    /// 获取远程 node列表
    async fn get_version_list_by_remote(&mut self);

    /// 查找node版本 通过远程
    async fn get_version_by_remote(&mut self) -> Option<&NodeVersionItem>;


    /// 下载node
    async fn download_node_by_remote(&mut self, version: &DownloadNodeItem);

    /// 从远程同步 node 版本 到本地
    async fn sync_node_by_remote(&mut self) -> DownloadNodeItem;

    /// 解压 本地node压缩包
    async fn unzip_node_file(&self, file_dir: &Path);

    /// 获取本地node版本
    async fn get_version_by_local(&mut self, version: &str) -> Option<String>;
}

#[async_trait]
impl NodeDispose for NsvCore {

    fn format_version(&self, version: &str) -> Result<String, NsvCoreError>{

        // 空字符串
        if version.len() == 0 {
            return Err(NsvCoreError::Empty);
        }

        // 正则校验
        let version_reg = create_node_version_vaildate_reg("");
        if !version_reg.is_match(version) {
            return Err(NsvCoreError::IllegalityVersion(version.to_string()));
        }

        let (char, ver) = version.split_at(1);
        if char == "v" {
            return Ok(ver.to_string());
        }
        return Ok(version.to_string());
    }
    fn get_local_node_dir_2_dir_entry(&self, version: &str) -> Option<DirEntry> {
        let version_reg = regex::Regex::new(&format!("^{}", version)).unwrap();
        for entry in read_dir(&self.context.node_dir).unwrap() {
            let entry = entry.unwrap();
            if version_reg.is_match(entry.file_name().to_str().unwrap()) {
                return Some(entry);
            }
        }
        return None;
    }

    fn set_version_target(&mut self, version: &str) -> Result<(), NsvCoreError> {
        if version.len() == 0 {
            return Err(NsvCoreError::Empty);
        }

        let target = match version {
            "lts" => Ok(VersionTarget::Lts),
            "latest" => Ok(VersionTarget::Latest),
            _ => {
                if !create_node_version_vaildate_reg("").is_match(version) {
                    return Err(NsvCoreError::IllegalityVersion(version.to_string()))
                }
                Ok(VersionTarget::Assign(version.to_string()))
            }
        };

        if target.is_err() {
            return Err(target.err().unwrap());
        };
        self.context.target = target.unwrap();
        Ok(())
    }

    async fn get_version_list_by_remote(&mut self) {
        if self.context.node_version_list.is_some() {
            return;
        }
        let url = format!("{}/index.json", self.config.origin);
        let resp = reqwest::get(url).await.unwrap();
        let resp_json: Vec<NodeVersionItem> = resp.json().await.unwrap();
        self.context.node_version_list = Some(resp_json);
    }

    async fn get_version_by_remote(&mut self) -> Option<&NodeVersionItem> {
        self.get_version_list_by_remote().await;
        match &self.context.target {
            VersionTarget::Lts => self
                .context
                .node_version_list
                .as_ref()
                .unwrap()
                .iter()
                .find(|item| match item.lts {
                    NodeLtsTarget::Str(_) => true,
                    _ => false,
                }),
            VersionTarget::Latest => self.context.node_version_list.as_ref().unwrap().first(),
            VersionTarget::Assign(version) => {
                let assign_version_reg = create_node_version_vaildate_reg(version);
                self.context
                    .node_version_list
                    .as_ref()
                    .unwrap()
                    .iter()
                    .find(|item| assign_version_reg.is_match(&*item.version))
            }
        }
    }


    async fn download_node_by_remote(&mut self, download_fine_info: &DownloadNodeItem) {
        download_file(&download_fine_info.url, &download_fine_info.target)
            .await
            .unwrap();
    }
    async fn sync_node_by_remote(&mut self) -> DownloadNodeItem {
        let file_name = format!(
            "node-v{}-{}-{}.{}",
            self.context.version, self.context.os, self.context.arch, self.context.rar_extension
        );
        let url = format!(
            "{}/v{}/{}",
            self.config.origin, self.context.version, file_name
        );
        let mut target = self.context.node_file.clone();
        target.push(&file_name);

        let download_fine_info = DownloadNodeItem {
            file_name,
            url,
            target,
        };

        println!("{:?}", download_fine_info);

        self.download_node_by_remote(&download_fine_info).await;

        return download_fine_info
    }

    async fn unzip_node_file(&self, file_dir: &Path) {
        let mut output_dir = self.context.temp.clone();
        ensure_dir(&output_dir).await.unwrap();
        unzip_file(file_dir, &output_dir).await.unwrap();


        let node_dir_file_name = file_dir.file_name().unwrap().to_str().unwrap().replace(&format!(".{}", self.context.rar_extension), "");

        output_dir.push(node_dir_file_name.clone());

        let mut node_dir = self.context.node_dir.clone();
        node_dir.push(self.context.version.clone());
        rename(&output_dir, &node_dir).await.unwrap();
    }

    async fn get_version_by_local(&mut self, version: &str) -> Option<String> {

        let version: Option<String> = match &self.context.target {
            // 输入 lts latest 等
            VersionTarget::Latest | VersionTarget::Lts => {
                let node_version_item = self.get_version_by_remote().await;
                if node_version_item.is_none() {
                    return None;
                }
                Some(node_version_item.unwrap().version.clone())
            }
            // 输入的是精准node版本
            _ => Some(version.to_string()),
        };

        if version.is_none() {
            return None;
        };

        let local_node_dir = self.get_local_node_dir_2_dir_entry(&version.unwrap());
        if local_node_dir.is_some() {
            Some(
                local_node_dir
                    .unwrap()
                    .file_name()
                    .to_str()
                    .unwrap()
                    .to_string(),
            )
        } else {
            None
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(untagged)]
pub enum NodeLtsTarget {
    Bool(bool),
    Str(String),
}

#[derive(Deserialize, Clone, Debug)]
pub struct NodeVersionItem {
    /// 版本
    pub version: String,

    /// 日期
    pub date: String,

    /// lts
    pub lts: NodeLtsTarget,

    /// 安全版本
    pub security: bool,
}

#[derive(Clone, Debug)]
pub struct DownloadNodeItem {
    pub url: String,
    pub file_name: String,
    pub target: PathBuf,
}
