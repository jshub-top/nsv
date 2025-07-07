use crate::core::NsvCore;
use async_trait::async_trait;
use download::NodeDisposeDownload;
use semver::Version;
use serde::Deserialize;
use std::path::PathBuf;
use util::dir::remove_symlink_dir;
use version::NodeDisposeVersion;

pub mod download;
pub mod version;

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
     * 自定义报错信息
     */
    Str(&'static str),

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
    NodeVersionLocalExist(String),

    /**
     * config key 不存在
     */
    ConfigKeyNotFound(String),
}

#[async_trait]
pub trait NodeDispose {
    /// 切换`node`版本
    async fn use_node(&self, version: &str, option: NsvUseNodeOption) -> Result<(), NsvCoreError>;
    /// 添加`node`版本
    async fn add_node(
        &mut self,
        version: &str,
        option: NsvAddNodeOption,
    ) -> Result<(), NsvCoreError>;
    /// 查看`node`版本
    async fn view_node_version(&mut self, version: Option<&str>) -> Result<(), NsvCoreError>;
}

pub struct NsvUseNodeOption {
    /// 确保版本存在 (当本地找不到自动下载)
    pub ensure: bool,
}

pub struct NsvAddNodeOption {
    /// 如果版本存在是否更新最新版本
    /// ```sh
    /// $ nsv add 18
    /// ```
    /// 当添加版本为`18`时 当前已添加的版本为`18.5.1`最新版本为`18.9.0` 如果为`true` 将会下载`18.9.0`
    ///
    pub upgrade: bool,
}

#[async_trait]
impl NodeDispose for NsvCore {
    async fn use_node(&self, version: &str, option: NsvUseNodeOption) -> Result<(), NsvCoreError> {
        // 转换成正常版本号
        let vers = self.formatter_version(version).await?;

        // 看一下本地有没有
        let mut vers = self.find_local_version(&vers).await;

        //没有就去远程找
        if vers.is_err() {
            // 如果 不需要去远程找 抛出错误
            if !option.ensure {
                return Err(NsvCoreError::NodeVersionLocalNotFound);
            }

            let _vers = vers.unwrap();
            let _vers = self.find_remote_version(&_vers).await?;
            vers = Ok(_vers.clone());
            self.download_node(&_vers).await?;
            self.unzip_node_file(&_vers).await?;
        };

        // 删除旧的 文件
        let mate_env_path = PathBuf::from(&self.context.shell_matefile_env);
        if let Err(e) = remove_symlink_dir(&mate_env_path).await {
            if e.kind() != std::io::ErrorKind::NotFound {
                panic!("{}", e)
            }
        }

        let vers = vers.unwrap();

        let vers_path = self.context.node_dir.join(vers);

        #[cfg(windows)]
        {
            use tokio::fs::symlink_dir;
            symlink_dir(&vers_path, &mate_env_path).await.unwrap();
        }

        #[cfg(unix)]
        {
            let mut vers_path = vers_path;
            use tokio::fs::symlink;
            // unix 系统的 node 可执行文件在 bin下面
            vers_path.push("bin");
            symlink(&vers_path, &mate_env_path).await.unwrap();
        }

        Ok(())
    }

    async fn add_node(
        &mut self,
        version: &str,
        option: NsvAddNodeOption,
    ) -> Result<(), NsvCoreError> {
        // 转换成正常版本号
        let vers = self.formatter_version(version).await?;
        // 看一下本地有没有
        let vers = self.find_local_version(&vers).await;
        self.download_dist_version().await?;

        if vers.is_ok() {
            // 如果不升级
            if !option.upgrade {
                return Err(NsvCoreError::NodeVersionLocalExist(vers.unwrap()));
            }

            let local_version = vers.unwrap();
            let remote_version = self.find_remote_version(version).await?;

            let local_vers = Version::parse(&local_version).unwrap();
            let remote_vers = Version::parse(&remote_version).unwrap();
            // 对比一下版本号如果 本地和远程一样 就是已存在
            if local_vers >= remote_vers {
                return Err(NsvCoreError::NodeVersionLocalExist(
                    local_version.to_string(),
                ));
            }
        }

        let vers = self.find_remote_version(version).await?;

        self.download_node(&vers).await.unwrap();
        self.unzip_node_file(&vers).await.unwrap();

        Ok(())
    }

    async fn view_node_version(&mut self, version: Option<&str>) -> Result<(), NsvCoreError> {
        match version {
            Some(version) => {
                // 转换成正常版本号
                let vers = self.formatter_version(version).await?;
                self.download_dist_version().await.unwrap();
                let node_version_list = self.context.node_version_list.clone();
                let item = node_version_list.iter().find(|item| item.version == vers);
                if item.is_none() {
                    return Err(NsvCoreError::IllegalityVersion(vers));
                }
                let item = item.unwrap();
                self.view_version_detail(item).await?;
            }
            None => {
                // 下载一下 node 版本列表
                self.download_dist_version().await.unwrap();
                self.view_version_list().await?;
            }
        }

        Ok(())
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

    /// 本地是否已安装
    pub is_installed: bool,

    /// module
    pub module: Option<String>,

    /// openssl版本
    pub openssl: Option<String>,

    /// zlib 版本
    pub zlib: Option<String>,

    /// uv 版本
    pub uv: Option<String>,

    /// v8 版本
    pub v8: Option<String>,

    /// npm 版本
    pub npm: Option<String>,
}

#[derive(Clone, Debug)]
pub struct DownloadNodeItem {
    pub url: String,
    pub file_name: String,
    pub target: PathBuf,
}
