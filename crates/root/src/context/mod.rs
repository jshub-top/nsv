use std::env;
use std::path::PathBuf;
use crate::core::node::{VersionTarget, NodeVersionItem};



#[derive(Debug, Clone)]
pub struct Context {
    /**
     * node压缩包扩展名 tar.xz 7z
     */
    pub rar_extension: &'static str,

    /**
     * node压缩包的文件名
     */
    pub file_name: String,

    /**
     * 当前node版本
     */
    pub node_version: String,

    /**
     * 工作目录
     */
    pub pwd: PathBuf,

    /**
     * 缓存路径
     */
    pub temp: PathBuf,

    /**
     * node压缩包路径
     */
    pub node_file: PathBuf,

    /**
     * node解压完成路径
     */
    pub node_dir: PathBuf,

    /**
     * node 版本标记
     */
    pub target: VersionTarget,

    /**
     * 在本地已存在
     */
    pub local_exist: bool,

    /**
     * 操作系统类型
     */
    pub os: String,

    /**
     * cpu类型
     */
    pub arch: String,

    /**
     * nodeitem
     */
    pub node_item: Option<NodeVersionItem>
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



        let mut current_dir = env::current_exe().unwrap();
        current_dir.pop();
        let pwd = current_dir.clone();

        current_dir.push("temp");
        let temp = current_dir.clone();

        current_dir.pop();
        current_dir.push("node_file");
        let node_file = current_dir.clone();

        current_dir.pop();
        current_dir.push("node_dir");
        let node_dir = current_dir.clone();


        Context {
            rar_extension,
            file_name: "".to_string(),
            node_version: "".to_string(),
            pwd,
            temp,
            node_file,
            node_dir,
            target: VersionTarget::None,
            local_exist: false,
            os: os.to_string(),
            arch: arch.to_string(),
            node_item: None,
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
