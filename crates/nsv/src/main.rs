use root::{core::NsvCore, config::Config};


#[tokio::main]
async fn main() {
    let core = NsvCore::build(Config::build(Box::new(|_config| {})));
}
