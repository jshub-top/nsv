mod add;
mod r#use;
use add::Add;
use async_trait::async_trait;
use r#use::Use;
use root::core::{ NsvCore};
use root::node;
use root::node::NsvCoreError;
use crate::print_log_err;

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
    pub async fn call(&self, core: &mut NsvCore) {
        match self {
            Self::Use(cmd) => cmd.call(core).await,
            Self::Add(cmd) => cmd.call(core).await,
        }
    }
}

#[async_trait]
pub trait Command {
    async fn call(&self, core: &mut NsvCore) {
        match self.apply(core).await {
            Ok(()) => (),
            Err(err) => {
                println!("err: {:?}", err);
                match err {
                    NsvCoreError::NodeItemExisted => {
                        println!("nsv: version is existed")
                    }
                    _ => {
                        core.context.node_version_list = None;
                        println!(
                            "err: {:?}\n{:?}\n{:?}",
                            err,
                            core.context,
                            core.config,
                        );
                        panic!()
                    }
                }
            },
        }
    }

    async fn apply(&self, core: &mut NsvCore) -> Result<(), NsvCoreError>;

    fn handle_err(&self, err: NsvCoreError, core: &mut NsvCore) {
        let err_s = format!("{:?}", err);
        print_log_err!("{}.\ncontext: {:?}", err_s, core.context);
        std::process::exit(1);
    }
}
