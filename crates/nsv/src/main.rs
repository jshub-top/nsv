mod cli;
mod command;
mod config;
use cli::parse;
use root::{config::Config, core::{NsvCore, init::Init}};

#[tokio::main]
async fn main() {
    let cli = parse();
    let mut config = Config::build();
    if cfg!(debug_assertions) {
        config.origin = "http://127.0.0.1:3000"
    }
    let mut nsv_core = NsvCore::build(config);
    nsv_core.init().await;
    cli.subcmd.call(cli.config, &mut nsv_core).await
}
