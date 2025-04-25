use async_trait::async_trait;
use root::{core::NsvCore, node::{NodeDispose, NsvAddNodeOption, NsvUseNodeOption}};

use crate::print_log_info;

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
        let option = NsvUseNodeOption {
            ensure: false
        };
        println!("{}", &self.version);
        core.use_node(&self.version, option).await?;
        Ok(())
    }
}

#[derive(Debug, Error)]
pub enum Error {}
