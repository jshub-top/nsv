use async_trait::async_trait;

use super::{NsvCore, node::{VersionError, NodeVersion as _}};



#[async_trait]
trait AddVersion {
    async fn add(&mut self, target: String) -> Result<(), VersionError>;
}

#[async_trait]
impl AddVersion for NsvCore {
    async fn add(&mut self, target: String) -> Result<(), VersionError> {
        self.vail_version(&target)?;
        let node_item = self.get_node_version_item(&target).await;

        match node_item {
            Err(e) => {

                if e != VersionError::Empyt {
                    return Err(e)
                }
            }
            Ok(node_item) => {

            }
        }



        Ok(())
    }
}
