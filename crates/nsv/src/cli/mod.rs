use root::{config::Config, core::NsvCore};

pub struct Cli {
    pub core: NsvCore,
}

impl Cli {
    pub fn build() -> Cli {
        Cli {
            core: NsvCore::build(Config::build(Box::new(|_config| {}))),
        }
    }
    pub fn run(&self) {
        println!("321")
    }
}
