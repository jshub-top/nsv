mod r#use;
mod add;
use async_trait::async_trait;
use r#use::Use;
use root::core::{NsvCore, node::NsvCoreError};
use add::Add;

use crate::{config::NsvConfig, print_log_err};

#[derive(clap::Parser, Debug)]
pub enum Commands {
    /// 修改node版本
    #[clap(name = "use", bin_name = "use")]
    Use(Use),

    /// 下载node版本
    #[clap(name = "add", bin_name = "add")]
    Add(Add),
}
impl Commands {
    pub async fn call(&self, config: NsvConfig, core: &mut NsvCore) {
        match self {
            Self::Use(cmd) => cmd.call(config, core).await,
            Self::Add(cmd) => cmd.call(config, core).await,
        }
    }
}

#[async_trait]
pub trait Command {
    async fn call(&self, config: NsvConfig, core: &mut NsvCore) {
        match self.apply(&config, core).await {
            Ok(()) => (),
            Err(err) => self.handle_err(err, &config, core),
        }
    }

    async fn apply(&self, config: &NsvConfig, core: &mut NsvCore) -> Result<(), NsvCoreError>;

    fn handle_err(&self, err: NsvCoreError, config: &NsvConfig, core: &mut NsvCore) {
        let err_s = format!("{:?}", err);
        print_log_err!("{}.\nconfig: {:?} \ncontext: {:?}", err_s, config, core.context);
        std::process::exit(1);
    }
}
