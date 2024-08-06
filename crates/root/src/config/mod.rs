use std::io::BufWriter;
use std::{env, fmt::Display};
use std::path::PathBuf;

use regex::Regex;
use tokio::io::AsyncWriteExt;
use tokio::{
    fs::{metadata, File},
    io::{AsyncBufReadExt, AsyncReadExt, BufReader},
};

#[derive(Debug, Clone)]
pub struct Config {
    pub origin: String,
    pub file_path: PathBuf,
}

impl Config {
    pub async fn build() -> Self {
        let mut config = Config::default();

        config.sync_config_2_npmrc("test", "test1").await;

        config
    }

    pub fn set_config(&mut self, key: &str, value: &str) {
        match key {
            "origin" => {
                self.origin = value.to_string();
            }
            _ => {}
        }
    }

    pub async fn sync_config_2_npmrc<T: Display>(&self, key: &str, value: T) {

        // 文件不存在 不继续往下走
        if metadata(&self.file_path).await.is_err() {
            File::create(&self.file_path).await.unwrap();
        }
        let config_file = File::open(&self.file_path).await.unwrap();
        let mut file_buf = BufReader::new(config_file);
        let mut file_content = String::new();
        file_buf.read_to_string(&mut file_content).await.unwrap();
        let file_lines: Vec<&str> = file_content.split("\n").collect();
        let line_index = file_lines.iter().position(|line| line.contains(key));







    }

    pub async fn parse_config_by_nsvrc(&mut self) {
        let file_path = &self.file_path;

        println!("config file path: {}", file_path.display());
        // 文件不存在 不继续往下走
        if metadata(file_path).await.is_err() {
            return;
        }

        let config_file = File::open(file_path).await.unwrap();
        let mut lines = BufReader::new(config_file).lines();
        while let Some(line) = lines.next_line().await.unwrap() {
            println!("line: {}", line);
            let config_vec = line.split("=").collect::<Vec<_>>();
            let key = config_vec[0];
            let value = config_vec[1];
            if key.is_empty() || value.is_empty() {
                panic!("config file format error");
            }
            self.set_config(config_vec[0], config_vec[1]);
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
        }
    }
}
