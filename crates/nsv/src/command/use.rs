use async_trait::async_trait;
use root::core::{NsvCore};

use thiserror::Error;
use root::node::NsvCoreError;
use super::Command;

#[derive(clap::Parser, Debug)]
pub struct Use {
    version: String,
}


#[async_trait]
impl Command for Use {
    async fn apply(&self, core: &mut NsvCore) -> Result<(), NsvCoreError> {


        Ok(())
    }
}

#[derive(Debug, Error)]
pub enum Error {}
