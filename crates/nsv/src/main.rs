mod cli;
mod command;
mod config;
use cli::parse;
use root::{config::Config, core::NsvCore};

#[tokio::main]
async fn main() {
    let cli = parse();
    let mut nsv_core = NsvCore::build(Config::build(Box::new(|config| {
        if cfg!(debug_assertions) {
            config.origin = "http://127.0.0.1:3000"
        }
    })));
    cli.subcmd.call(cli.config, &mut nsv_core).await
}
