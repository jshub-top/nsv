mod cli;
mod command;
mod config;
use cli::parse;
use root::{config::Config, core::NsvCore};

#[tokio::main]
async fn main() {
    let cli = parse();
    let nsv_core = NsvCore::build(Config::build(Box::new(|_| {})));
    cli.subcmd.call(cli.config, &nsv_core)
}
