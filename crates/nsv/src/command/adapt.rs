use super::Command;
use async_trait::async_trait;
use root::node::NsvCoreError;
use root::{core::NsvCore, node::NodeDispose};
use thiserror::Error;
use tokio::fs::read_to_string;

#[derive(clap::Parser, Debug)]
pub struct Adapt {}

#[async_trait]
impl Command for Adapt {
    async fn apply(&self, core: &mut NsvCore) -> Result<(), NsvCoreError> {
        if !core.config.adapt {
            return Ok(());
        }
        let file = core
            .config
            .adapt_version_match
            .clone()
            .unwrap_or(".nsvrc".to_string());

        let version = read_to_string(file).await;

        if version.is_err() {
            return Ok(());
        }

        let version = version.unwrap();
        let version = version.trim();

        core.set_version_target(version)?;
        let mut local_node_version = core.get_version_by_local().await;
        if local_node_version.is_none() && core.config.auto {
            let remote_node_version = core.get_version_by_remote().await;
            if remote_node_version.is_none() {
                return Err(NsvCoreError::NodeVersionRemoteNotFound);
            };
            let remote_node_version = remote_node_version.unwrap().clone();
            let version = remote_node_version.get_version();
            let download_node_info = core
                .sync_node_by_remote(&version)
                .await;
            core.unzip_node_file(&download_node_info.target).await;
            local_node_version = Some(version);
        }

        let local_node_version = local_node_version.as_ref().unwrap();
        core.sync_mate_file_by_version(local_node_version).await;

        Ok(())
    }
}

#[derive(Debug, Error)]
pub enum Error {}
