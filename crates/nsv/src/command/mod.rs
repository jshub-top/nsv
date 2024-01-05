mod r#use;
mod add;
use async_trait::async_trait;
use r#use::Use;
use root::core::NsvCore;
use add::Add;

use crate::config::NsvConfig;

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
    type Error: std::error::Error;
    async fn call(&self, config: NsvConfig, core: &mut NsvCore) {
        match self.apply(&config, core).await {
            Ok(()) => (),
            Err(err) => self.handle_err(err, &config),
        }
    }

    async fn apply(&self, config: &NsvConfig, core: &mut NsvCore) -> Result<(), Self::Error>;

    fn handle_err(&self, err: Self::Error, config: &NsvConfig) {
        let err_s = format!("{err}");
        println!("err: {}. config: {:?}", err_s, config);
        std::process::exit(1);
    }
}
