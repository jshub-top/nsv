use crate::print_log_info;

use super::Command;
use async_trait::async_trait;
use root::{core::NsvCore, node::NsvCoreError};

#[derive(clap::Parser, Debug)]
pub struct Config {
    #[clap(subcommand)]
    pub subcommand: ConfigSubCommand,
}

#[async_trait]
impl Command for Config {
    async fn apply(&self, core: &mut NsvCore) -> Result<(), NsvCoreError> {
        match &self.subcommand {
            ConfigSubCommand::Set(set) => {
                set.apply(core).await?;
            }
            ConfigSubCommand::Get(get) => {
                get.apply(core).await?;
            }
            ConfigSubCommand::List(get) => {
                get.apply(core).await?;
            }
        }
        Ok(())
    }
}

#[derive(clap::Parser, Debug)]
pub enum ConfigSubCommand {
    #[clap(name = "set", bin_name = "set", alias = "s")]
    Set(ConfigSubSet),
    #[clap(name = "get", bin_name = "get", alias = "g")]
    Get(ConfigSubGet),
    #[clap(name = "list", bin_name = "list", alias = "l")]
    List(ConfigSubList),
}

#[derive(clap::Parser, Debug)]
pub struct ConfigSubSet {
    pub key: String,
    pub value: String,
    #[arg(short, long)]
    global: bool,
}

#[async_trait]
impl Command for ConfigSubSet {
    async fn apply(&self, core: &mut NsvCore) -> Result<(), NsvCoreError> {
        match self.global {
            true => core.config.set_global(&self.key, &self.value),
            false => core.config.set(&self.key, &self.value),
        }
        print_log_info!("set config success: {}", &self.key);
        Ok(())
    }
}

#[derive(clap::Parser, Debug)]
pub struct ConfigSubGet {
    pub key: String,
}

#[async_trait]
impl Command for ConfigSubGet {
    async fn apply(&self, core: &mut NsvCore) -> Result<(), NsvCoreError> {
        let value = core.config.get::<String>(&self.key);
        print_log_info!("{}={}", &self.key, value);
        Ok(())
    }
}

#[derive(clap::Parser, Debug)]
pub struct ConfigSubList {
    #[arg(short, long)]
    all: bool,
}

#[async_trait]
impl Command for ConfigSubList {
    async fn apply(&self, core: &mut NsvCore) -> Result<(), NsvCoreError> {
        print_log_info!("config list: \n{}", &core.config.display());
        Ok(())
    }
}
