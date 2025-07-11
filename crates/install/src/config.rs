use std::env;

use util::platform::get_home_dir;

pub struct Config {
    pub origin: String,
    pub nsv_home: String,
}

impl Config {
    pub fn new() -> Self {
        let nsv_home = env::var("NSV_HOME").unwrap_or(
            get_home_dir()
                .unwrap()
                .join(".nsv")
                .to_str()
                .unwrap()
                .to_string(),
        );
        Self {
            origin: env::var("GITHUB_API_ORIGIN").unwrap_or("https://api.github.com".to_string()),
            nsv_home,
        }
    }
}
