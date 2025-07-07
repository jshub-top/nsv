mod context;
mod core;
mod shell;
mod config;


#[tokio::main]
async fn main() {
    let app = core::Main::new();

    app.run().await;
}
