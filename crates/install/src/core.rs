use std::{env, path::PathBuf, process::Command};

use anyhow::Result;
use tokio::fs::write;
use util::{fs::ensure_dir, platform::Env::Shell};

use crate::{config::Config, context::Context, shell::get_shell_profile_path};

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

    pub async fn run(&self) -> Result<()> {
        if self.installed() {
            println!("nsv is installed");
            return Ok(());
        }

        self.set_nsv_profile().await?;

        let shell_profile_path = get_shell_profile_path(&self.context.shell);

        Ok(())
    }

    pub async fn set_nsv_profile(&self) -> Result<()> {
        let nsv_profile_path = self.get_nsv_profile_path();
        let content = self.get_nsv_profile_content();
        println!("content: {:?}", content);
        ensure_dir(nsv_profile_path.parent().unwrap()).await?;
        write(&nsv_profile_path, content).await?;
        Ok(())
    }

    pub async fn set_shell_profile(&self) -> Result<()> {
        let shell_profile_path = get_shell_profile_path(&self.context.shell);
        let content = self.get_shell_profile_content(&shell_profile_path);
        ensure_dir(shell_profile_path.parent().unwrap()).await?;
        write(&shell_profile_path, content).await?;
        Ok(())
    }

    pub fn get_nsv_profile_path(&self) -> PathBuf {
        let nsv_config_path = self.context.nsv_config_path.clone();
        match self.context.shell {
            Shell::Bash | Shell::Zsh | Shell::Sh => nsv_config_path.join("config.sh"),
            Shell::Fish => nsv_config_path.join("config.fish"),
            Shell::PowershellDesktop | Shell::PowershellCore => nsv_config_path.join("config.ps1"),
        }
    }

    pub fn get_nsv_profile_content(&self) -> &'static str {
        match self.context.shell {
            Shell::Bash | Shell::Zsh | Shell::Sh => {
                r#"
    timestamp=$(date +%s%3N)
    export NSV_HOME=~/.nsv
    export NSV_MATEFILE=$NSV_HOME/temp/$$_$timestamp
    export PATH=$NSV_MATEFILE:$NSV_HOME/temp/default:$NSV_HOME:$PATH
    nsv adapt
                "#
            }
            Shell::Fish => {
                r#"
    set timestamp (date +%s%3N)
    set -gx NSV_HOME ~/.nsv
    set -gx NSV_MATEFILE $NSV_HOME/temp/"$fish_pid"_"$timestamp"
    set -gx PATH $NSV_MATEFILE $NSV_HOME/temp/default $NSV_HOME $PATH
    nsv adapt
                "#
            }
            Shell::PowershellDesktop | Shell::PowershellCore => {
                r#"
    $timestamp=Get-Date -UFormat %s
    $Env:NSV_MATEFILE="$Env:NSV_HOME\temp\$timestamp"
    $Env:Path="$Env:NSV_MATEFILE;$Env:NSV_HOME\temp\default;$Env:NSV_HOME;$Env:Path"
    nsv adapt
                "#
            } //         Shell::Cmd => {
              //             r#"
              // @echo off
              // set timestamp=%date:~10,4%%date:~4,2%%date:~7,2%%time:~0,2%%time:~3,2%%time:~6,2%
              // set NSV_MATEFILE=%NSV_HOME%\temp%timestamp%
              // set PATH=%NSV_MATEFILE%;%NSV_HOME%\temp\default;%NSV_HOME%;%PATH%
              // nsv adapt
              //             "#
              //         }
        }
    }

    pub fn get_shell_profile_content(&self, nsv_profile_path: &PathBuf) -> String {
        match self.context.shell {
            Shell::Bash | Shell::Zsh | Shell::Sh => r#"
# nsv
$nsv_profile_path = ~/.config/nsv/config.sh
[[ -f $nsv_profile_path ]] && . $nsv_profile_path
# nsv end
            "#
            .to_string(),
            Shell::Fish => r#"
# nsv
$nsv_profile_path = ~/.config/nsv/config.fish
[[ -f $nsv_profile_path ]] && . $nsv_profile_path
# nsv end
            "#
            .to_string(),
            Shell::PowershellDesktop | Shell::PowershellCore => {
                format!(
                    r#"
# nsv
$nsv_profile_path = {}
if(Test-Path -Path $nsv_profile_path) {{
    . $nsv_profile_path
}}
            "#,
                    nsv_profile_path.to_str().unwrap()
                )
            }
        }
    }

    pub fn get_shell_profile_path(self) -> PathBuf {
        match self.context.shell {
            Shell::Bash => PathBuf::from(env::var("HOME").unwrap()).join(".bashrc"),
            Shell::Zsh => PathBuf::from(env::var("HOME").unwrap()).join(".zshrc"),
            Shell::Sh => PathBuf::from(env::var("HOME").unwrap()).join(".bashrc"),
            Shell::Fish => PathBuf::from(env::var("HOME").unwrap())
                .join(".config/fish")
                .join("config.fish"),
            Shell::PowershellDesktop => PathBuf::from(env::var("USERPROFILE").unwrap())
                .join("Documents\\WindowsPowerShell")
                .join("Microsoft.PowerShell_profile.ps1"),
            Shell::PowershellCore => PathBuf::from(env::var("USERPROFILE").unwrap())
                .join("Documents\\PowerShell")
                .join("Microsoft.PowerShell_profile.ps1"),
            // Shell::Cmd => {
            //     panic!("cmd is not supported")
            // }
        }
    }
    pub fn installed(&self) -> bool {
        if let Ok(output) = Command::new("nsv").args(&["-V"]).output() {
            return output.status.success();
        }

        false
    }

    // pub fn get_
}
