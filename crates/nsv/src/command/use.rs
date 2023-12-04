use root::core::NsvCore;

use crate::config::NsvConfig;

use super::Command;

#[derive(clap::Parser, Debug)]
pub struct Use {}

impl Command for Use {
    type Error = Error;
    fn apply(&self, config: &NsvConfig, core: &NsvCore) -> Result<(), Error>{

        Ok(())
    }
}

pub enum Error {

}
