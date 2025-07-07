use std::env;



pub struct Config {
    pub origin: String,
}

impl Config {
    pub fn new() -> Self {
        Self {
            origin: env::var("GITHUB_ORIGIN").unwrap_or("https://github.com".to_string()),
        }
    }
}
