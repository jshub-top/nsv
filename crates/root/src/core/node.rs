use std::path::PathBuf;

use async_trait::async_trait;
use regex::Regex;
use serde::Deserialize;
use sha256::try_async_digest;
use tokio::{
    fs::{create_dir_all, read_dir, remove_dir_all, remove_file, rename, File},
    io::AsyncReadExt,
};

use crate::util::download::{download_file, unzip_file};

use super::NsvCore;

#[derive(PartialEq, Debug, Clone)]
pub enum VersionTarget {
    Lts,
    Latest,
    Assign,
    None,
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
    Empyt,

    /**
     * 非法版本
     */
    IllegalityVersion,

    /**
     * 版本不存在
     */
    NotFound,

    /**
     * node 文件在未找到
     */
    NodeItemFIleNotFound,

    /**
     * node版本在本地已存在
     */
    NodeItemExisted,
}

#[async_trait]
pub trait NodeVersion {
    /**
     * 获取远程 node版本列表
     */
    async fn origin_node_version(&self) -> Vec<NodeVersionItem>;

    /**
     * 获取最新lts node版本
     */
    async fn get_lts_version(&self) -> Option<NodeVersionItem>;

    /**
     * 获取最新node版本
     */
    async fn get_latest_version(&self) -> Option<NodeVersionItem>;

    /**
     * 获取输入的 node版本 从本地和远程获取
     */
    async fn get_assign_version(&self) -> Option<NodeVersionItem>;

    /**
     * 验证输入版本号是否有效
     */
    fn vail_version(&mut self, version: &str) -> Result<(), NsvCoreError>;

    /**
     * 根据 输入的版本号获取 node版本
     */
    async fn get_node_version_item(&self) -> Result<NodeVersionItem, NsvCoreError>;

    /**
     * 本地是否存 当前node版本
     */
    fn assign_local_node_exist(&self, node_item: &NodeVersionItem) -> bool;

    /**
     * 下载node版本到本地
     */
    async fn download_node(
        &self,
        node_item: &NodeVersionItem,
    ) -> Result<(), Box<dyn std::error::Error>>;

    /**
     * 获取 node 文件路径
     */
    fn get_node_file_path(&self, node_item: &NodeVersionItem) -> PathBuf;

    /**
     * 获取 node 文件名称
     */
    fn get_node_file_name(&self, node_item: &NodeVersionItem) -> String;

    /**
     * 获取 node 解压之后的 文件夹名字
     */
    fn get_node_file_unzip_dir_name(&self, node_item: &NodeVersionItem) -> String;

    /**
     * 确保 sha256 文件存在
     */
    async fn ensure_node_sha256_file_exist(&self, node_item: &NodeVersionItem);

    /**
     * 获取 本地 sha256 文件路径
     */
    fn get_sha256_file_path(&self, node_item: &NodeVersionItem) -> PathBuf;

    /**
     * 使用sha256验证本地 node 版本
     */
    async fn vail_local_node_item(&self, node_item: &NodeVersionItem) -> bool;

    /**
     * 下载和验证node版本
     */
    async fn vail_and_download_file(&self, node_item: &NodeVersionItem) -> bool;

    /**
     * 解压node 文件
     */
    async fn unzip_node_item(&self, node_item: &NodeVersionItem) -> Result<(), NsvCoreError>;
}

#[async_trait]
impl NodeVersion for NsvCore {
    async fn unzip_node_item(&self, node_item: &NodeVersionItem) -> Result<(), NsvCoreError> {
        let node_file_dir = self.get_node_file_path(node_item);

        if !node_file_dir.exists() {
            return Err(NsvCoreError::NodeItemFIleNotFound);
        }

        let unzip_dir = self.context.node_dir.clone().join(&node_item.version);

        if unzip_dir.exists() {
            remove_dir_all(&unzip_dir).await.unwrap();
            create_dir_all(&unzip_dir).await.unwrap();
        }

        unzip_file(&node_file_dir, &unzip_dir.parent().unwrap())
            .await
            .unwrap();

        let unzip_file_dir_name = self
            .context
            .node_dir
            .clone()
            .join(self.get_node_file_unzip_dir_name(node_item));

        rename(unzip_file_dir_name, unzip_dir).await.unwrap();

        Ok(())
    }
    async fn vail_and_download_file(&self, node_item: &NodeVersionItem) -> bool {
        let local_node_item_dir = self.get_node_file_path(node_item);
        if !local_node_item_dir.exists() {
            #[cfg(debug_assertions)]
            self.download_node(node_item).await.unwrap();
        }
        let mut vail_ok = self.vail_local_node_item(node_item).await;
        if !vail_ok {
            // 如果验证失败 就删除下载
            remove_file(local_node_item_dir).await.unwrap();
            self.download_node(node_item).await.unwrap();
            vail_ok = self.vail_local_node_item(node_item).await;
        }
        return vail_ok;
    }
    async fn vail_local_node_item(&self, node_item: &NodeVersionItem) -> bool {
        self.ensure_node_sha256_file_exist(node_item).await;

        let node_file_sha256_dir = self.get_sha256_file_path(node_item);

        let mut sha_file_content = "".to_string();
        File::open(node_file_sha256_dir)
            .await
            .unwrap()
            .read_to_string(&mut sha_file_content)
            .await
            .unwrap();
        let node_item_file_dir = self.get_node_file_path(node_item);
        let node_file_sha256 = try_async_digest(node_item_file_dir).await.unwrap();
        return sha_file_content.contains(&node_file_sha256);
    }
    fn get_sha256_file_path(&self, node_item: &NodeVersionItem) -> PathBuf {
        let node_file_dir = self.context.node_file.clone();

        return node_file_dir
            .join(&node_item.version)
            .join("SHASUMS256.txt");
    }
    async fn ensure_node_sha256_file_exist(&self, node_item: &NodeVersionItem) {
        let node_file_sha256_dir = self.get_sha256_file_path(node_item);

        if node_file_sha256_dir.exists() {
            return;
        }

        let sha256_file_url = format!(
            "{}/dist/{}/SHASUMS256.txt",
            self.config.origin, &node_item.version
        );

        create_dir_all(&node_file_sha256_dir.parent().unwrap())
            .await
            .unwrap();
        download_file(&sha256_file_url, &node_file_sha256_dir)
            .await
            .unwrap();
    }

    fn get_node_file_path(&self, node_item: &NodeVersionItem) -> PathBuf {
        let node_file_dir = self.context.node_file.clone();

        return node_file_dir
            .join(&node_item.version)
            .join(self.get_node_file_name(node_item));
    }
    fn get_node_file_name(&self, node_item: &NodeVersionItem) -> String {
        // node-v20.10.0-darwin-x64.tar.xz
        return format!(
            "node-{}-{}-{}.{}",
            &node_item.version, self.context.os, self.context.arch, self.context.rar_extension
        );
    }
    fn get_node_file_unzip_dir_name(&self, node_item: &NodeVersionItem) -> String {
        self.get_node_file_name(node_item)
            .replace(format!(".{}", self.context.rar_extension).as_str(), "")
    }
    async fn download_node(
        &self,
        node_item: &NodeVersionItem,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let version = &node_item.version;
        // https://nodejs.org/dist/v20.10.0/node-v20.10.0-darwin-x64.tar.xz
        let download_url = format!(
            "{}/dist/{}/{}",
            self.config.origin,
            version,
            self.get_node_file_name(node_item)
        );
        let node_file_dir = self.get_node_file_path(node_item);
        create_dir_all(node_file_dir.parent().unwrap())
            .await
            .unwrap();
        download_file(&download_url, &node_file_dir).await?;
        Ok(())
    }

    async fn origin_node_version(&self) -> Vec<NodeVersionItem> {
        let node_origin = self.config.origin;
        let node_verison_url_origin = node_origin.to_string() + "/dist/index.json";
        let res = reqwest::get(&node_verison_url_origin)
            .await
            .expect(&format!("request error: {} ", &node_verison_url_origin));
        let version_list = res.json::<Vec<NodeVersionItem>>().await;
        return version_list.unwrap();
    }

    async fn get_lts_version(&self) -> Option<NodeVersionItem> {
        let version_list = self.origin_node_version().await;

        for item in version_list {
            match item.lts {
                NodeVersionLts::Type(_) => return Some(item),
                NodeVersionLts::Bool(_) => {}
            }
        }

        return None;
    }

    async fn get_latest_version(&self) -> Option<NodeVersionItem> {
        let version_list = self.origin_node_version().await;
        let item = version_list[0].clone();
        return Some(item);
    }

    async fn get_assign_version(&self) -> Option<NodeVersionItem> {
        if self.context.target != VersionTarget::Assign {
            return None;
        }
        let mut version_regexp_str = format!("^v?{}.", self.context.node_version);
        if version_regexp_str.contains("..") {
            version_regexp_str.pop();
        }
        let assign_version_regexp = Regex::new(&version_regexp_str).unwrap();

        let mut _node_item = None;

        let mut dir = read_dir(&self.context.node_dir).await.unwrap();
        while let Some(entry) = dir.next_entry().await.unwrap() {
            if !entry.path().is_dir() {
                continue;
            }
            let os_dir_name = entry.file_name();
            let version_str = os_dir_name.to_str().unwrap();
            let is_match = assign_version_regexp.is_match(version_str);
            if !is_match {
                continue;
            }

            _node_item = Some(NodeVersionItem {
                version: version_str.to_string(),
                date: "".to_string(),
                lts: NodeVersionLts::Bool(false),
            })
        }

        if let None = _node_item {
            let version_list = self.origin_node_version().await;
            _node_item = version_list
                .iter()
                .find(|node_item| return assign_version_regexp.is_match(&node_item.version))
                .cloned();
        }
        return _node_item;
    }

    fn vail_version(&mut self, version: &str) -> Result<(), NsvCoreError> {
        // 如果传入版本为空
        if version == "" {
            return Err(NsvCoreError::Empyt);
        }
        match version {
            "lts" => {
                self.context.target = VersionTarget::Lts;
            }

            "latest" => {
                self.context.target = VersionTarget::Latest;
            }
            _ => {
                self.context.target = VersionTarget::Assign;
                self.context.node_version = version.to_string();
                let version_list = version.split(".").collect::<Vec<&str>>();

                for v in version_list {
                    if let Err(_) = v.parse::<i32>() {
                        return Err(NsvCoreError::IllegalityVersion);
                    }
                }
            }
        }

        return Ok(());
    }

    async fn get_node_version_item(&self) -> Result<NodeVersionItem, NsvCoreError> {
        let mut version_item: Option<NodeVersionItem> = None;
        match self.context.target {
            VersionTarget::Lts => {
                version_item = self.get_lts_version().await;
            }

            VersionTarget::Latest => {
                version_item = self.get_latest_version().await;
            }
            VersionTarget::Assign => {
                if let None = version_item {
                    version_item = self.get_assign_version().await;
                }
            }
            _ => {}
        };

        if let Some(node_item) = version_item {
            return Ok(node_item);
        }

        Err(NsvCoreError::Empyt)
    }

    fn assign_local_node_exist(&self, node_item: &NodeVersionItem) -> bool {
        let node_dir = self.context.node_dir.clone();
        let node_item_dir = node_dir.join(&node_item.version);
        return node_item_dir.exists();
    }
}

#[derive(Deserialize, Clone, Debug)]
pub struct NodeVersionItem {
    pub version: String,
    pub date: String,
    // pub files: Vec<String>,
    // pub npm: Option<String>,
    // pub v8: String,
    // pub uv: Option<String>,
    // pub zlib: Option<String>,
    // pub openssl: Option<String>,
    // pub modules: Option<String>,
    pub lts: NodeVersionLts,
    // pub security: bool,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum NodeVersionLts {
    Type(String),
    Bool(bool),
}

#[cfg(test)]
mod tests {
    use crate::{
        config::Config,
        core::node::{NodeVersion, NsvCoreError},
    };

    use super::*;

    #[test]
    fn test_auth_version() {
        let config = Config::build();
        let mut core = NsvCore::build(config);
        assert!(matches!(core.vail_version(""), Err(NsvCoreError::Empyt),));
        assert!(matches!(
            core.vail_version("a.1.2"),
            Err(NsvCoreError::IllegalityVersion)
        ));
        assert!(matches!(core.vail_version("1.1.2"), Ok(())));
    }
}
