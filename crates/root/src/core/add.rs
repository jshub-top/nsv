use async_trait::async_trait;

use super::{
    node::{NodeVersion, NsvCoreError},
    NsvCore,
};

#[async_trait]
pub trait AddVersion {
    async fn add_version(&mut self, target: String) -> Result<(), NsvCoreError>;
}

#[async_trait]
impl AddVersion for NsvCore {
    async fn add_version(&mut self, target: String) -> Result<(), NsvCoreError> {
        self.vail_version(&target)?;
        let node_item = self.get_node_version_item().await?;
        self.context.node_item = Some(node_item.clone());

        let node_exist = self.assign_local_node_exist(&node_item);

        // 如果在本地存在这个版本
        if node_exist {
            return Err(NsvCoreError::NodeItemExisted);
        }

        self.vail_and_download_file(&node_item).await;
        self.unzip_node_item(&node_item).await.unwrap();
        return Ok(());
    }
}
