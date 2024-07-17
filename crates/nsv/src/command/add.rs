use async_trait::async_trait;
use thiserror::Error;
use root::core::NsvCore;

use super::Command;
use root::node::{NodeDispose, NsvCoreError};

#[derive(clap::Parser, Debug)]
pub struct Add {
    version: String,
}

#[async_trait]
impl Command for Add {
    async fn apply(&self, core: &mut NsvCore) -> Result<(), NsvCoreError> {
        let version = core.format_version(&self.version)?;
        core.set_version_target(&version)?;
        let local_node_version = core.get_version_by_local(&version).await;
        if local_node_version.is_some() {
            return Err(NsvCoreError::NodeVersionLocalExist)
        }
        drop(local_node_version);

        let mut remote_node_version = None;

        {
            remote_node_version = core.get_version_by_remote().await
        }

        if remote_node_version.is_none() {
            return Err(NsvCoreError::NodeVersionRemoteNotFound);
        };
        let remote_node_version = remote_node_version.unwrap();

        core.context.version = core.format_version(&remote_node_version.version)?;

        let download_node_info = core.sync_node_by_remote().await;

        core.unzip_node_file(&download_node_info.target).await;

        Ok(())
    }
}

#[derive(Debug, Error)]
pub enum Error {}
