use async_trait::async_trait;

use super::{
    node::{NodeVersion, NsvCoreError},
    NsvCore,
};


#[async_trait]
pub trait UseVersion {
    async fn use_version(&mut self, version: String) -> Result<(), NsvCoreError>;
}

#[async_trait]
impl UseVersion for NsvCore {
    async fn use_version(&mut self, version: String) -> Result<(), NsvCoreError> {

        // 验证 版本是否有效
        self.vail_version(&version)?;

        let _node_item = self.get_node_version_item().await;


        Ok(())
    }
}
