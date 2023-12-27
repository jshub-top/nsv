
#[derive(Debug, Clone)]
pub struct Config {
    pub origin: &'static str,
    // pub version: String,
}

impl Config {
    pub fn build() -> Self {
        let config = Config {
            origin: "https://nodejs.org",
        };
        config
    }

    pub fn set_config(&mut self, call_fn: Box<dyn FnOnce(&mut Config) -> ()>)  {
        call_fn(self);
    }
}
