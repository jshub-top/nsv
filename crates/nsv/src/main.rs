mod cli;
mod command;
mod config;
mod log;
use cli::parse;
use config::parse_config;
use root::core::{init::Init, NsvCore};

#[tokio::main]
async fn main() {
    let cli = parse();

    let mut nsv_core = NsvCore::build(parse_config());
    nsv_core.init().await;
    cli.subcommand.call(&mut nsv_core).await;
}
