pub struct Context {
    pub rar_extension: &'static str,
    pub download_url: &'static str,
    pub file_name: &'static str,
    pub node_version: &'static str,

}

impl Context {
    pub fn build() -> Context {
        // https://nodejs.org/dist/v20.9.0/node-v20.9.0-win-x86.7z
        #[cfg(any(target_os = "linux", target_os = "macos"))]
        let rar_extension = "tar.xz";
        #[cfg(target_os = "windows")]
        let rar_extension = "7z";
        Context {
            rar_extension,
            download_url: "",
            file_name: "",
            node_version: "",
        }
    }
}
