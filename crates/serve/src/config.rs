use std::{env, sync::Arc};

pub struct Config {
    pub port: String,
    pub host: String,
    pub db_url: String,
    pub server_irl: String,
}

impl Config {
    pub fn build() -> Arc<Self> {
        dotenvy::dotenv().ok();

        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL env var get error");
        let port = env::var("PORT").expect("PORT env var get error");
        let host = env::var("HOST").expect("HOST env var get error");
        let server_irl = format!("{host}:{port}");
        Arc::new(Config {
            db_url,
            port,
            host,
            server_irl,
        })
    }
}
