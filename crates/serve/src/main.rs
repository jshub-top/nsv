use app::App;


mod db;
mod router;
mod app;
mod controller;
mod util;
mod config;
mod service;
mod model;



#[tokio::main]
async fn main() {
    let config = config::Config::build();

    let app = App::build(config).await;
    app.serve.await.unwrap()
}
