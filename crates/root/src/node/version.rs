
use async_trait::async_trait;
use tokio::fs::read_dir;

use crate::core::NsvCore;

use super::{NodeLtsTarget, NsvCoreError};

#[async_trait]
pub trait NodeDisposeVersion {
    /// 查找本地 node 版本
    async fn find_local_version(&self, version: &str) -> Result<String, NsvCoreError>;

    /// 查找远程 node 版本
    async fn find_remote_version(&self, version: &str) -> Result<String, NsvCoreError>;

    /// 格式化 用户输入的版本
    async fn formatter_version(&self, version: &str) -> Result<String, NsvCoreError>;
}

#[async_trait]
impl NodeDisposeVersion for NsvCore {
    async fn find_local_version(&self, version: &str) -> Result<String, NsvCoreError> {
        let node_dir = self.context.node_dir.as_path();
        let mut local_node_dirs = read_dir(&node_dir).await.unwrap();
        while let Ok(Some(entry)) = local_node_dirs.next_entry().await {
            let file_name = entry.file_name();
            if let Some(name) = file_name.to_str() {
                // 如果是以 输入版本开头的
                if name.starts_with(version) {
                    return Ok(name.to_string());
                }
            }
        }
        return Err(NsvCoreError::NodeVersionLocalNotFound);
    }

    async fn find_remote_version(&self, version: &str) -> Result<String, NsvCoreError> {
        let version_list = self.context.node_version_list.clone();

        let current_version_item = version_list.iter().find(|item| {
            let (_, remote_version) = item.version.split_at(1);
            remote_version.starts_with(version)
        });

        if current_version_item.is_none() {
            return Err(NsvCoreError::NodeVersionRemoteNotFound);
        }

        Ok(current_version_item.unwrap().version.clone())
    }

    async fn formatter_version(&self, version: &str) -> Result<String, NsvCoreError> {
        match version {
            "lts" => {
                let current_version_item =
                    self.context
                        .node_version_list
                        .iter()
                        .find(|item| match item.lts {
                            // 当 lts是字符串时候就可以了
                            NodeLtsTarget::Str(_) => true,
                            _ => false,
                        });

                if current_version_item.is_none() {
                    return Err(NsvCoreError::NodeVersionRemoteNotFound);
                }

                return Ok("lts".to_string());
            }
            "latest" => {
                // 最新版本就获取 最新的呢个 版本
                let current_version_item = self.context.node_version_list.get(0);

                if current_version_item.is_none() {
                    return Err(NsvCoreError::NodeVersionRemoteNotFound);
                }

                return Ok("latest".to_string());
            }
            _ => {
                let (char, _) = version.split_at(1);
                if char == "v" {
                    return Ok(version.to_string())
                }

                Ok(format!("v{version}"))
            }
        }
    }
}
