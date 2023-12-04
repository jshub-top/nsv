use clap::Parser;
use root::core::NsvCore;

use crate::command::Commands;
use crate::config::NsvConfig;



#[derive(clap::Parser, Debug)]
#[clap(name = "nsv", version = env!("CARGO_PKG_VERSION"), bin_name = "nsv")]
pub struct Cli {
    // pub core: NsvCore,

    #[clap(flatten)]
    pub config: NsvConfig,

    #[clap(subcommand)]
    pub subcmd: Commands,
}

pub fn parse() -> Cli {
    Cli::parse()
}


