use crate::{config::Config, context::Context};
pub mod node;

pub struct NsvCore {
    pub config: Config,
    pub context: Context,
}

impl NsvCore {
    pub fn build(config: Config) -> NsvCore {
        NsvCore {
            config,
            context: Context::build(),
        }
    }
}
