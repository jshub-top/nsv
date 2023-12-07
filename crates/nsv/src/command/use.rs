use async_trait::async_trait;
use root::core::{NsvCore, r#use::UseVersion};

use crate::config::NsvConfig;
use thiserror::Error;

use super::Command;

#[derive(clap::Parser, Debug)]
pub struct Use {
    version: String,
}


#[async_trait]
impl Command for Use {
    type Error = Error;
    async fn apply(&self, _config: &NsvConfig, core: &mut NsvCore) -> Result<(), Error> {
        if let Err(_) = core.use_version(self.version.clone()).await {
            println!("err")
        }

        println!("{}", self.version);
        Ok(())
    }
}

#[derive(Debug, Error)]
pub enum Error {}
