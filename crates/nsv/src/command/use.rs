use async_trait::async_trait;
use root::{core::NsvCore, node::NodeDispose};

use super::Command;
use root::node::NsvCoreError;
use thiserror::Error;

#[derive(clap::Parser, Debug)]
pub struct Use {
    version: String,
}

#[async_trait]
impl Command for Use {
    async fn apply(&self, core: &mut NsvCore) -> Result<(), NsvCoreError> {
        core.set_version_target(&self.version)?;
        let local_node_version = core.get_version_by_local().await;
        if local_node_version.is_none() {
            return Err(NsvCoreError::NodeVersionLocalNotFound)
        }
        let local_node_version = local_node_version.as_ref().unwrap();
        core.sync_mate_file_by_version(local_node_version).await;
        Ok(())
    }
}

#[derive(Debug, Error)]
pub enum Error {}
