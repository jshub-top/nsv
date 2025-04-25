use std::{
    sync::Arc,
    time::{Duration, SystemTime},
};

use async_trait::async_trait;
use tokio::{
    fs::{read_to_string, remove_file, write},
    spawn,
};

use crate::core::NsvCore;

use super::{NodeVersionItem, NsvCoreError};

#[async_trait]
pub trait NodeDisposeDownload {
    async fn download_node(&self, version: &str) -> Result<(), NsvCoreError>;
    async fn download_dist_version(&mut self) -> Result<Arc<Vec<NodeVersionItem>>, NsvCoreError>;
}

#[async_trait]
impl NodeDisposeDownload for NsvCore {
    async fn download_node(&self, version: &str) -> Result<(), NsvCoreError> {
        Ok(())
    }

    async fn download_dist_version(&mut self) -> Result<Arc<Vec<NodeVersionItem>>, NsvCoreError> {
        if self.context.node_version_list.len() != 0 {
            return Ok(self.context.node_version_list.clone());
        }

        let dist_version_path = self.context.nsv_home.join("version.json");

        // 如果 本地存在 就用本地的
        if dist_version_path.exists() {
            let file_meta = dist_version_path.metadata().unwrap();

            let file_create_time = file_meta.created().unwrap();
            let current_time = SystemTime::now();
            let time_difference = current_time
                .duration_since(file_create_time)
                .unwrap_or_default();

            let is_recent =
                time_difference <= Duration::from_secs(self.config.index_json_file_effect_time);

            if is_recent {
                let file_content = read_to_string(dist_version_path).await.unwrap();
                let version_list =
                    serde_json::from_str::<Vec<NodeVersionItem>>(&file_content).unwrap();
                let json_arc = Arc::new(version_list);
                self.context.node_version_list = json_arc.clone();
                return Ok(json_arc.clone());
            }
        }

        let url = format!("{}/index.json", self.config.origin);
        let resp = reqwest::get(url).await.unwrap();
        // 缓存到本地
        let resp_byt = resp.bytes().await.unwrap();
        let resp_json = serde_json::from_slice(&resp_byt).unwrap();
        self.context.node_version_list = Arc::new(resp_json);

        let defer = async move || {
            remove_file(&dist_version_path).await.unwrap();
            write(&dist_version_path, resp_byt).await.unwrap();
        };

        spawn(defer());

        Ok(self.context.node_version_list.clone())
    }
}
