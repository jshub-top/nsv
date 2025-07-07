use std::path::PathBuf;

use util::platform::{get_arch, get_is_admin, get_os, Arch, Env::{get_shell, Shell}, OS};


pub struct Context {

    /// 操作系统
    pub os: OS,

    /// 架构
    pub arch: Arch,

    /// 当前工作目录
    pub pwd: PathBuf,

    /// 是否为管理员
    pub admin: bool,

    /// 当前shell
    pub shell: Shell,
}

impl Context {
    pub fn new() -> Self {

        let os = get_os().unwrap();
        let arch = get_arch().unwrap();
        let shell = get_shell().unwrap();



        Self {
            os,
            arch,
            pwd: std::env::current_dir().unwrap(),
            admin: get_is_admin(),
            shell,
        }
    }

    pub fn get_nsv_profile_content(&self) -> &'static str {

        match self.shell {

            Shell::Bash | Shell::Zsh | Shell::Sh => {
                r#"
timestamp=$(date +%s%3N)
export NSV_HOME=$NSV_HOME
export NSV_MATEFILE=$NSV_HOME/temp/$timestamp
export PATH=$NSV_MATEFILE:$NSV_HOME/temp/default:$NSV_HOME:$PATH
nsv adapt
                "#
            },
            Shell::Fish => {
                r#"
set timestamp (date +%s%3N)
set -gx NSV_HOME $NSV_HOME
set -gx NSV_MATEFILE $NSV_HOME/temp/$timestamp
set -gx PATH $NSV_MATEFILE $NSV_HOME/temp/default $NSV_HOME $PATH
nsv adapt
                "#
            },
            Shell::Powershell => {
                r#"
$timestamp=Get-Date -UFormat %s
$Env:NSV_MATEFILE="$Env:NSV_HOME\temp\$timestamp"
$Env:Path="$Env:NSV_MATEFILE;$Env:NSV_HOME\temp\default;$Env:NSV_HOME;$Env:Path"
nsv adapt
                "#
            },
            Shell::Cmd => {
                r#"
@echo off
set timestamp=%date:~10,4%%date:~4,2%%date:~7,2%%time:~0,2%%time:~3,2%%time:~6,2%
set NSV_MATEFILE=%NSV_HOME%\temp%timestamp%
set PATH=%NSV_MATEFILE%;%NSV_HOME%\temp\default;%NSV_HOME%;%PATH%
nsv adapt
                "#
            }
        }
    }

}
