use url::Url;

#[derive(clap::Parser, Debug)]
pub struct NsvConfig {

    // /// 标签
    // #[clap(
    //     long,
    //     env = "TARGET",
    //     short = 't',
    //     hide_env_values = true,
    //     value_parser = ["lts", "current", "latest"],
    // )]
    // pub target: String,

    /// 下载node源地址
    #[clap(
        long,
        env = "ORIGIN",
        default_value = "https://nodejs.org/dist",
        global = true,
        hide_env_values = true,
        short = 'o',
    )]
    pub origin: Url,


}
