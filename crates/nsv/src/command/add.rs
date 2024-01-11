use async_trait::async_trait;
use root::core::{node::NsvCoreError, NsvCore};

use super::Command;
use crate::{config::NsvConfig, print_log_1};
use root::core::add::AddVersion;
use thiserror::Error;

#[derive(clap::Parser, Debug)]
pub struct Add {
    version: String,
}

#[async_trait]
impl Command for Add {
    async fn apply(&self, _config: &NsvConfig, core: &mut NsvCore) -> Result<(), NsvCoreError> {
        match core.add_version(self.version.clone()).await {

            Ok(()) => {
                let node_item = core.context.node_item.clone().unwrap();
                print_log_1!("nsv: {} add successlfy!", node_item.version)
            }
            Err(err) => {
                if err == NsvCoreError::NodeItemExisted {
                    let node_item = core.context.node_item.clone().unwrap();
                    print_log_1!("{} already exist!", node_item.version);
                    return Ok(());
                }

                return Err(err);


            }
        }
        Ok(())
    }
}

#[derive(Debug, Error)]
pub enum Error {}
