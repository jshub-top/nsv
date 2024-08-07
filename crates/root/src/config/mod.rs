use std::env;
use std::path::PathBuf;

use tokio::fs::write;
use tokio::{
    fs::{metadata, File},
    io::{AsyncBufReadExt, BufReader},
};

#[derive(Debug, Clone)]
pub struct Config {
    /// 源
    pub origin: String,

    /// 配置文件路径
    pub file_path: PathBuf,

    /// 是否适配
    pub adapt: bool,

    /// 是否自动处理
    pub auto: bool,

    /// 适配版本匹配
    pub adapt_version_match: Option<String>,
}

impl Config {
    pub async fn build() -> Self {
        let mut config = Config::default();

        // global config
        config
            .parse_config_by_nsvrc(&config.file_path.clone())
            .await;

        // project config
        // config.parse_config_by_nsvrc(&Path::new(".nsvrc").to_path_buf()).await;

        config
    }

    pub fn set_config(&mut self, key: &str, value: &str) {
        match key {
            "origin" => {
                self.origin = value.to_string();
            }
            "adapt" => {
                self.adapt = value.parse::<bool>().unwrap();
            }
            "auto" => {
                self.auto = value.parse::<bool>().unwrap();
            }
            _ => {}
        }
    }

    pub async fn sync_config_2_npmrc(&self) {
        let config_file_content = vec![
            format!("origin={}", &self.origin),
            format!("adapt={}", self.adapt),
            format!("auto={}", self.adapt),
        ];
        write(&self.file_path, config_file_content.join("\n"))
            .await
            .unwrap();
    }

    pub async fn parse_config_by_nsvrc(&mut self, file_path: &PathBuf) {
        // 文件不存在 不继续往下走
        if metadata(file_path).await.is_err() {
            return;
        }

        let config_file = File::open(file_path).await.unwrap();
        let mut lines = BufReader::new(config_file).lines();
        while let Some(line) = lines.next_line().await.unwrap() {
            let config_vec = line.split("=").collect::<Vec<_>>();
            let key = config_vec[0];
            let value = config_vec[1];
            if !key.is_empty() && !value.is_empty() {
                self.set_config(config_vec[0], config_vec[1]);
            }
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        #[cfg(windows)]
        let user_home = env::var("USERPROFILE").unwrap();
        #[cfg(unix)]
        let user_home = env::var("HOME").unwrap();
        let mut user_home = PathBuf::from(user_home);
        user_home.push(".nsvrc");
        Self {
            origin: env::var("NSV_ORIGIN").unwrap_or("https://nodejs.org/dist".to_string()),
            file_path: user_home,
            adapt: true,
            auto: true,
            adapt_version_match: env::var("NSV_ADAPT_MATCH").ok(),
        }
    }
}
