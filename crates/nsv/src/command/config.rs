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
    async fn apply(&self, core: &mut NsvCore) -> Result<(), NsvCoreError>{
        match &self.subcommand {
            ConfigSubCommand::Set(set) => {
                set.apply(core).await?;
            }
            ConfigSubCommand::Get(get) => {
                get.apply(core).await?;
            }
        }
        Ok(())
    }
}

#[derive(clap::Parser, Debug)]
pub enum ConfigSubCommand {
    #[clap(name = "set", bin_name = "set")]
    Set(ConfigSubSet),
    #[clap(name = "get", bin_name = "get")]
    Get(ConfigSubGet),
}


#[derive(clap::Parser, Debug)]
pub struct ConfigSubSet {
    pub key: String,
    pub value: String,
}

#[async_trait]
impl Command for ConfigSubSet {
    async fn apply(&self, core: &mut NsvCore) -> Result<(), NsvCoreError>{
        core.config.set_config(&self.key, &self.value);
        core.config.sync_config_2_npmrc().await;
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
    async fn apply(&self, core: &mut NsvCore) -> Result<(), NsvCoreError>{
        let value = core.config.get_config(&self.key)?;
        print_log_info!("{}={}", &self.key, value);
        Ok(())
    }
}
