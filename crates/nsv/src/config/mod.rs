
use root::config::Config;

pub async fn parse_config() -> Config {
    let config = Config::build();
    return config;
}
