use std::env;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Context {
    /**
     * node压缩包扩展名
     */
    pub rar_extension: &'static str,

    /**
     * 远程node文件的下载地址
     */
    pub download_url: &'static str,

    /**
     * node压缩包的文件名
     */
    pub file_name: &'static str,

    /**
     * 当前node版本
     */
    pub node_version: &'static str,

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
}

impl Context {
    pub fn build() -> Context {
        // https://nodejs.org/dist/v20.9.0/node-v20.9.0-win-x86.7z
        #[cfg(any(target_os = "linux", target_os = "macos"))]
        let rar_extension = "tar.xz";
        #[cfg(target_os = "windows")]
        let rar_extension = "7z";

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
            download_url: "",
            file_name: "",
            node_version: "",
            pwd,
            temp,
            node_file,
            node_dir,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::context::Context;

    #[test]
    fn test_context() {
        println!("{:?}", Context::build())
    }
}
