
#[derive(Debug, Clone)]
pub struct Config {
    pub origin: &'static str,
    // pub version: String,
}

impl Config {
    pub fn build(call_fn: Box<dyn FnOnce(&Config) -> ()>) -> Self {
        let mut config = Config {
            origin: "https://nodejs.org/dist",
        };

        call_fn(&mut config);

        config
    }

}
