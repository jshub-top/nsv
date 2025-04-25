use super::Command;
use async_trait::async_trait;
use root::core::NsvCore;
use root::node::NsvCoreError;
use thiserror::Error;

#[derive(clap::Parser, Debug)]
pub struct Adapt {}

#[async_trait]
impl Command for Adapt {
    async fn apply(&self, core: &mut NsvCore) -> Result<(), NsvCoreError> {
        Ok(())
    }
}

#[derive(Debug, Error)]
pub enum Error {}
