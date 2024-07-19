use std::{
    env::{self},
    path::Path,
};

use async_trait::async_trait;
use root::core::NsvCore;

use super::Command;
use root::node::NsvCoreError;
use thiserror::Error;

#[derive(clap::Parser, Debug)]
pub struct Install {}

#[async_trait]
impl Command for Install {
    async fn apply(&self, core: &mut NsvCore) -> Result<(), NsvCoreError> {







        Ok(())
    }
}

#[derive(Debug, Error)]
pub enum Error {}
