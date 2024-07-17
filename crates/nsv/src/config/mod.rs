
use root::config::Config;

pub fn parse_config() -> Config {
    let mut config = Config::build();

    #[cfg(debug_assertions)]
    {
        config.origin = "http://127.0.0.1:3001/cdn";
    }


    return config;
}
