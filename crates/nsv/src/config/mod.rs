
use root::config::Config;

pub async fn parse_config() -> Config {
    let mut config = Config::build().await;

    #[cfg(debug_assertions)]
    {
        config.origin = "http://127.0.0.1:3001/cdn".to_string();
    }


    return config;
}
