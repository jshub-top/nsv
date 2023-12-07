use async_trait::async_trait;
use serde::Deserialize;

use super::NsvCore;

#[async_trait]
pub trait NodeVersion {
    async fn origin_node_version(&self) -> Vec<NodeVersionItem>;

    async fn get_lts_version(&self) -> Option<NodeVersionItem>;
    async fn get_latest_version(&self) -> Option<NodeVersionItem>;
}

#[async_trait]
impl NodeVersion for NsvCore {
    async fn origin_node_version(&self) -> Vec<NodeVersionItem> {
        let node_origin = self.config.origin;
        let res = reqwest::get(node_origin.to_string() + "/dist/index.json")
            .await
            .unwrap();
        let version_list = res.json::<Vec<NodeVersionItem>>().await;
        println!("111 {:?}", version_list);
        return version_list.unwrap();
    }

    async fn get_lts_version(&self) -> Option<NodeVersionItem> {
        let version_list = self.origin_node_version().await;

        for item in version_list {
            if item.lts {
                return Some(item);
            }
        }

        return None;
    }
    async fn get_latest_version(&self) -> Option<NodeVersionItem> {
        let version_list = self.origin_node_version().await;
        let item = version_list[0].clone();
        return Some(item);
    }
}

#[derive(Deserialize, Clone, Debug)]
pub struct NodeVersionItem {
    pub version: String,
    pub date: String,
    pub files: Vec<String>,
    pub npm: Option<String>,
    pub v8: String,
    pub uv: Option<String>,
    pub zlib: Option<String>,
    pub openssl: Option<String>,
    pub modules: Option<String>,
    pub lts: bool,
    pub security: bool,
}
