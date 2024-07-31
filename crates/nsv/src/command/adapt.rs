use async_trait::async_trait;
use root::core::NsvCore;
use thiserror::Error;

use crate::print_log_1;

use super::Command;
use root::node::NsvCoreError;

#[derive(clap::Parser, Debug)]
pub struct Adapt {
}

#[async_trait]
impl Command for Adapt {
    async fn apply(&self, core: &mut NsvCore) -> Result<(), NsvCoreError> {
        print_log_1!("adapt {}", "功能待开发");

        Ok(())
    }
}

#[derive(Debug, Error)]
pub enum Error {}
