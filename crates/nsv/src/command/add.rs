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
        core.sync_version_by_str(&self.version).await
    }
}

#[derive(Debug, Error)]
pub enum Error {}
