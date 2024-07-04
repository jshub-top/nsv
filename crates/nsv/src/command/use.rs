use async_trait::async_trait;
use root::core::{NsvCore, r#use::UseVersion, node::NsvCoreError};

use thiserror::Error;

use super::Command;

#[derive(clap::Parser, Debug)]
pub struct Use {
    version: String,
}


#[async_trait]
impl Command for Use {
    async fn apply(&self, core: &mut NsvCore) -> Result<(), NsvCoreError> {
        if let Err(err) = core.use_version(self.version.clone()).await {
            println!("err1: {:?}", err)
        }

        Ok(())
    }
}

#[derive(Debug, Error)]
pub enum Error {}
