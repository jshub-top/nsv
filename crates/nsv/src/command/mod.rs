mod r#use;
use r#use::Use;
use root::core::NsvCore;

use crate::config::NsvConfig;

#[derive(clap::Parser, Debug)]
pub enum Commands {
    Use(Use),
}
impl Commands {
    pub fn call(&self, config: NsvConfig, core: &NsvCore) {
        match self {
            Self::Use(cmd) => cmd.call(config, core),
        }
    }
}

pub trait Command {
    type Error: std::error::Error;
    fn call(&self, config: NsvConfig, core: &NsvCore) {
        match self.apply(&config, core) {
            Ok(()) => (),
            Err(err) => self.handle_err(err, &config),
        }
    }

    fn apply(&self, config: &NsvConfig, core: &NsvCore) -> Result<(), Self::Error>;

    fn handle_err(&self, err: Self::Error, config: &NsvConfig) {
        let err_s = format!("{err}");
        println!("err: {}. config: {:?}", err_s, config);
        std::process::exit(1);
    }
}
