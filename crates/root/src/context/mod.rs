use std::env;
use std::env::current_dir;
use std::path::PathBuf;
use crate::node::{NodeVersionItem, VersionTarget};

#[derive(Debug, Clone)]
pub struct Context {
    /// node压缩包扩展名 tar.xz 7z
    pub rar_extension: &'static str,


    /// node压缩包的文件名
    pub file_name: String,


    /// 操作的node版本
    pub version: String,

    /// 工作目录
    pub pwd: PathBuf,

    /// nsv home
    pub nsv_home: PathBuf,

    /// 缓存路径
    pub temp: PathBuf,


    /// node压缩包路径
    pub node_file: PathBuf,


    /// node解压完成路径
    pub node_dir: PathBuf,


    /// node 版本标记
    pub target: VersionTarget,

    /// 操作系统类型
    pub os: String,


    /// cpu类型
    pub arch: String,

    /// 远程 nodejs 列表
    pub node_version_list: Option<Vec<NodeVersionItem>>,
}

impl Context {
    pub fn build() -> Context {
        // https://nodejs.org/dist/v20.9.0/node-v20.9.0-win-x86.7z


        #[cfg(any(target_os = "linux", target_os = "macos"))]
        let rar_extension = "tar.xz";
        #[cfg(target_os = "windows")]
        let rar_extension = "7z";


        #[cfg(target_os = "windows")]
        let os = "win";
        #[cfg(target_os = "linux")]
        let os = "linux";
        #[cfg(target_os = "macos")]
        let os = "darwin";

        #[cfg(target_arch = "x86_64")]
        let arch = "x64";
        #[cfg(target_arch = "aarch64")]
        let arch = "arm64";


        let mut current_exe_dir = env::current_exe().unwrap();
        current_exe_dir.pop();
        let nsv_home = current_exe_dir.clone();
        current_exe_dir.push("temp");
        let temp = current_exe_dir.clone();
        current_exe_dir.pop();

        current_exe_dir.push("node_file");
        let node_file = current_exe_dir.clone();
        current_exe_dir.pop();

        current_exe_dir.push("node_dir");
        let node_dir = current_exe_dir.clone();


        Context {
            file_name: "".to_string(),
            rar_extension,
            version: "".to_string(),
            nsv_home,
            pwd: current_dir().unwrap(),
            temp,
            node_file,
            node_dir,
            target: VersionTarget::Latest,
            os: os.to_string(),
            arch: arch.to_string(),
            node_version_list: None,
        }
    }
}

#[cfg(test)]
mod test {
    // use crate::context::Context;

    #[test]
    fn test_context() {
        // println!("{:?}", Context::build())
    }
}
