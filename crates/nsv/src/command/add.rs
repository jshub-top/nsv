use async_trait::async_trait;
use root::core::NsvCore;

use crate::config::NsvConfig;
use thiserror::Error;
use root::core::add::AddVersion;
use super::Command;

#[derive(clap::Parser, Debug)]
pub struct Add {
    version: String,
}

#[async_trait]
impl Command for Add {
    type Error = Error;
    async fn apply(&self, _config: &NsvConfig, core: &mut NsvCore) -> Result<(), Error> {
        if let Err(err) = core.add_version(self.version.clone()).await {
            println!("err1: {:?}", err)
        }
        Ok(())
    }
}

#[derive(Debug, Error)]
pub enum Error {}
