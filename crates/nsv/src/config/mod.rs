use url::Url;

#[derive(clap::Parser, Debug)]
pub struct NsvConfig {
    #[clap(
        long,
        env = "ORIGIN",
        default_value = "https://nodejs.org/dist",
        global = true,
        hide_env_values = true
    )]
    pub origin: Url,
}
