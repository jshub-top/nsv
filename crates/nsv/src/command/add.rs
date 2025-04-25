use async_trait::async_trait;
use root::{core::NsvCore, node::{NodeDispose, NsvAddNodeOption}};
use thiserror::Error;

use crate::print_log_info;

use super::Command;
use root::node::NsvCoreError;

#[derive(clap::Parser, Debug)]
pub struct Add {
    /// add node version.
    version: String,
}

#[async_trait]
impl Command for Add {
    async fn apply(&self, core: &mut NsvCore) -> Result<(), NsvCoreError> {
        let option = NsvAddNodeOption {
            upgrade: false
        };

        core.add_node(&self.version, option).await?;

        Ok(())
    }
}

#[derive(Debug, Error)]
pub enum Error {}
