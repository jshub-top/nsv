use async_trait::async_trait;
use root::core::{node::NsvCoreError, NsvCore};

use super::Command;
use crate::{ print_log_1};
use root::core::add::AddVersion;
use thiserror::Error;

#[derive(clap::Parser, Debug)]
pub struct Add {
    version: String,
}

#[async_trait]
impl Command for Add {
    async fn apply(&self, core: &mut NsvCore) -> Result<(), NsvCoreError> {

        core.add_version(self.version.clone()).await?;

        if let Some(node_item) = &core.context.node_item {
            print_log_1!("nsv: {} add successlfy!", node_item.version);
        }
        Ok(())

    }
}

#[derive(Debug, Error)]
pub enum Error {}
