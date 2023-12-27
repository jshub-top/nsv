use async_trait::async_trait;

use super::{
    node::{NodeVersion, VersionError},
    NsvCore,
};


#[async_trait]
pub trait UseVersion {
    async fn use_version(&mut self, version: String) -> Result<(), VersionError>;
}

#[async_trait]
impl UseVersion for NsvCore {
    async fn use_version(&mut self, version: String) -> Result<(), VersionError> {

        // 验证 版本是否有效
        self.vail_version(&version)?;

        println!("{:?}", self.context.target);
        let node_item = self.get_node_version_item(&version).await;


        println!("{:?}",node_item);
        Ok(())
    }
}
