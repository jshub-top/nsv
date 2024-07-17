#[derive(Debug, Clone)]
pub struct Config {
    pub origin: &'static str,
}

impl Config {
    pub fn build() -> Self {
        let config = Config::default();
        config
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            origin: "https://nodejs.org/dist",
        }
    }
}
