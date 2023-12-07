use async_trait::async_trait;

use crate::{config::Config, context::Context};

pub mod r#use;
pub mod init;
pub mod node;

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
