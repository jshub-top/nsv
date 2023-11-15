mod cli;
use cli::Cli;

#[tokio::main]
async fn main() {
    let cli_instance = Cli::build();
    cli_instance.run();
}
