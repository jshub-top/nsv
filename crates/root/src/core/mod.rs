use crate::{config::Config, context::Context};

pub mod node;
pub mod init;

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
