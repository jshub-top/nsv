use async_trait::async_trait;
use thiserror::Error;
use root::core::NsvCore;

use crate::print_log_info;

use super::Command;
use root::node::{NodeDispose, NsvCoreError};

#[derive(clap::Parser, Debug)]
pub struct Add {
    /// add node version.
    version: String,
}

#[async_trait]
impl Command for Add {
    async fn apply(&self, core: &mut NsvCore) -> Result<(), NsvCoreError> {
        core.set_version_target(&self.version)?;
        let local_node_version = core.get_version_by_local().await;
        if local_node_version.is_some() {
            return Err(NsvCoreError::NodeVersionLocalExist(local_node_version.unwrap()))
        }
        drop(local_node_version);

        let remote_node_version = core.get_version_by_remote().await;


        if remote_node_version.is_none() {
            return Err(NsvCoreError::NodeVersionRemoteNotFound);
        };

        let remote_node_version = remote_node_version.unwrap().clone();



        let download_node_info = core.sync_node_by_remote(&remote_node_version.get_version()).await;



        core.unzip_node_file(&download_node_info.target).await;

        print_log_info!("{} added", &remote_node_version.version);
        Ok(())
    }
}

#[derive(Debug, Error)]
pub enum Error {}
