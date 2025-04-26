use async_trait::async_trait;
use root::{
    core::NsvCore,
    node::{NodeDispose, NsvUseNodeOption},
};

use crate::print_log_info;

use super::Command;
use root::node::NsvCoreError;
use thiserror::Error;

#[derive(clap::Parser, Debug)]
pub struct Use {
    version: Option<String>,
}

#[async_trait]
impl Command for Use {
    async fn apply(&self, core: &mut NsvCore) -> Result<(), NsvCoreError> {
        let version = match &self.version {
            Some(version) => version.clone(),
            None => core.config.get("node"),
        };

        if version.is_empty() {
            return Err(NsvCoreError::NodeVersionLocalNotFound);
        };

        let option = NsvUseNodeOption { ensure: false };
        core.use_node(&version, option).await?;
        Ok(())
    }
}

#[derive(Debug, Error)]
pub enum Error {}
