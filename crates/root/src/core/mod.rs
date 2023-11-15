use crate::config::Config;
pub mod node;


pub struct NsvCore {
    pub config: Config
}


impl NsvCore {
    pub fn build(config: Config) -> NsvCore {


        NsvCore {
            config
        }
    }
}
