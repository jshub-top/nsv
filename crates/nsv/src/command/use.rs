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
    async fn apply(&self, _core: &mut NsvCore) -> Result<(), NsvCoreError> {
        // core.format_version_str(&self.version)?;

        // let version = core.find_version_by_local(&self.version).await;

        // if version.is_none() {
        //     return Err(NsvCoreError::NodeVersionLocalNotFound)
        // }

        Ok(())
    }
}

#[derive(Debug, Error)]
pub enum Error {}
