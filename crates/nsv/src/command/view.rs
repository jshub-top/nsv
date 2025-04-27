use async_trait::async_trait;
use root::core::NsvCore;
use thiserror::Error;


use super::Command;
use root::node::NsvCoreError;

#[derive(clap::Parser, Debug)]
pub struct View {
    /// add node version.
    version: Option<String>,

    /// view
    #[arg(short, long)]
    local: bool,
}

#[async_trait]
impl Command for View {
    async fn apply(&self, core: &mut NsvCore) -> Result<(), NsvCoreError> {





        Ok(())
    }
}

#[derive(Debug, Error)]
pub enum Error {}
