use crate::{config::Config, context::{self, Context}};

pub struct Main {
    pub context: Context,
    pub config: Config,
}

impl Main {
    pub fn new() -> Self {
        Self {
            context: Context::new(),
            config: Config::new(),
        }
    }

    pub async fn run(&self) {
        println!("run");
        println!("shell: {:?}", self.context.shell);
        println!("{}", self.context.get_nsv_profile_content());
    }
}



