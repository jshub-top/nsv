use std::path::PathBuf;

use regex::Regex;
use util::platform::{get_arch, get_is_admin, get_os, Arch, Env::{get_shell, Shell}, OS};

use crate::shell::get_nsv_config_path;


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

    /// version
    pub version: String,

    /// nsv config path
    pub nsv_config_path: PathBuf,

    /// shell profile nsv installed reg
    pub nsv_installed_reg: Regex,

}

impl Context {
    pub fn new() -> Self {

        let os = get_os().unwrap();
        let arch = get_arch().unwrap();
        let shell = get_shell().unwrap();

        let nsv_config_path = get_nsv_config_path();

        Self {
            os,
            arch,
            pwd: std::env::current_dir().unwrap(),
            admin: get_is_admin(),
            shell,
            version: env!("CARGO_PKG_VERSION").to_string(),
            nsv_config_path,
            nsv_installed_reg: Regex::new(r"# nsv[\s\S]*?#\s*nsv end").unwrap(),
        }
    }
}
