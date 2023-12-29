use async_trait::async_trait;
use serde::Deserialize;
use tokio::fs::read_dir;

use super::NsvCore;

#[derive(PartialEq, Debug, Clone)]
pub enum VersionTarget {
    Lts,
    Latest,
    Assign,
    None,
}

#[derive(PartialEq, Debug)]
pub enum VersionError {
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
}

#[async_trait]
pub trait NodeVersion {
    async fn origin_node_version(&self) -> Vec<NodeVersionItem>;

    async fn get_lts_version(&self) -> Option<NodeVersionItem>;
    async fn get_latest_version(&self) -> Option<NodeVersionItem>;

    fn vail_version(&mut self, version: &str) -> Result<(), VersionError>;
    async fn get_node_version_item(&self, version: &str) -> Result<NodeVersionItem, VersionError>;
    async fn search_local_node_version(&self, version: &str) -> Option<NodeVersionItem>;
}

#[async_trait]
impl NodeVersion for NsvCore {
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

    fn vail_version(&mut self, version: &str) -> Result<(), VersionError> {
        // 如果传入版本为空
        if version == "" {
            return Err(VersionError::Empyt);
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
                let version_list = version.split(".").collect::<Vec<&str>>();

                for v in version_list {
                    if let Err(_) = v.parse::<i32>() {
                        return Err(VersionError::IllegalityVersion);
                    }
                }
            }
        }

        return Ok(());
    }

    async fn get_node_version_item(&self, version: &str) -> Result<NodeVersionItem, VersionError> {
        let mut version_item = None;
        match self.context.target {
            VersionTarget::Lts => {
                version_item = self.get_lts_version().await;
            }

            VersionTarget::Latest => {
                version_item = self.get_latest_version().await;
            }
            VersionTarget::Assign => {
                self.search_local_node_version(version).await;
            }
            _ => {}
        };

        if let Some(node_item) = version_item {
            return Ok(node_item);
        }

        Err(VersionError::Empyt)
    }

    async fn search_local_node_version(&self, _version: &str) -> Option<NodeVersionItem> {
        let mut dir = read_dir(&self.context.node_dir).await.unwrap();
        while let Some(entry) = dir.next_entry().await.unwrap() {
            let file_type = entry.file_type().await.unwrap();
            print!("{:?}", file_type)
        }
        return None;
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
        core::node::{NodeVersion, VersionError},
    };

    use super::*;

    #[test]
    fn test_auth_version() {
        let config = Config::build();
        let mut core = NsvCore::build(config);
        assert!(matches!(core.vail_version(""), Err(VersionError::Empyt),));
        assert!(matches!(
            core.vail_version("a.1.2"),
            Err(VersionError::IllegalityVersion)
        ));
        assert!(matches!(core.vail_version("1.1.2"), Ok(())));
    }
}
