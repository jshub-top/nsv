

use crate::{config::Config, context::Context};

pub mod add;
pub mod init;
pub mod r#use;



#[derive(Debug, Clone)]
pub struct NsvCore {
    pub config: Config,
    pub context: Context,
}

impl NsvCore {
    pub fn build(config: Config) -> NsvCore {
        let instance = NsvCore {
            config,
            context: Context::build(),
        };

        instance
    }
}

