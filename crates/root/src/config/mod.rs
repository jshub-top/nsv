use ini::Ini;
use std::env::{self, current_dir};
use std::fs::read_to_string;
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct Config {
    // / 源
    // pub origin: String,

    // / 配置文件路径
    // pub file_path: PathBuf,

    // / 添加node版本时候是否自动更新node版本到最新 比如 add 18 本地 18.5 新版18.6 将自动更新到 18.6
    // pub upgrade: bool,

    // / 是否通过 `npmrc` 自动读取node版本进行修改
    // pub adapt: bool,

    // / 默认node版本
    // pub node: String,

    // index.json 有效期是多久
    // pub index_json_file_effect_time: u64
    def_config: Ini,
    user_config: Ini,
    global_config: Ini,
    rc: PathBuf,
    global_nsvrc_path: PathBuf,
}

impl Config {
    pub fn build() -> Self {
        let default_config = format!(
            r#"
            index_json_file_effect_time="{}"
            origin="https://nodejs.org/dist"
            upgrade="true"
            adapt="false"
        "#,
            60 * 60 * 5,
        );

        //从 pwd 寻找顶层 nsvrc 文件
        let mut pwd = current_dir().unwrap();
        pwd.push(".nsvrc");
        let mut rc = None;
        let config_file_list = [".nsvrc", ".nvmrc", ".node-version"];
        while pwd.pop() {
            let rc_path = config_file_list.iter().find_map(| file | {
                let file_dir = pwd.join(file);
                if file_dir.exists() {
                    return Some(file_dir)
                };
                None
            });
            if rc_path.is_some() {
                rc = rc_path;
                break;
            }
        }

        let pwd = current_dir().unwrap();


        let user_config = match rc.as_ref() {
            Some(path) => match Ini::load_from_file(path) {
                Ok(config) => config,
                Err(_) => {
                    // 这里是为了兼容旧版本
                    // ```ini
                    // .nsvrc
                    // 20
                    // ```
                    // 旧版本很简单的就一个 20 新版本使用ini 来存储信息
                    let mut ini = Ini::new();

                    let value = read_to_string(path).unwrap();

                    ini.set_to(None::<&str>, "node".to_string(), value);

                    ini
                }
            },
            None => Ini::new(),
        };

        let rc = rc.unwrap_or_else(|| pwd.join(".nsvrc"));

        #[cfg(windows)]
        let user_home = env::var("USERPROFILE").unwrap();
        #[cfg(unix)]
        let user_home = env::var("HOME").unwrap();

        let global_nsvrc_path = PathBuf::from(user_home).join(".nsvrc");
        let global_config = match Ini::load_from_file(&global_nsvrc_path) {
            Ok(config) => config,
            Err(_) => {
                let ini = Ini::new();
                ini
            }
        };

        Self {
            def_config: Ini::load_from_str(&default_config).unwrap(),
            user_config,
            global_config,
            rc,
            global_nsvrc_path,
        }
    }

    pub fn get<T>(&self, key: &str) -> T
    where
        T: FromStr,
        <T as FromStr>::Err: std::fmt::Debug,
    {
        // Try user config first
        if let Some(value) = self.user_config.get_from(None::<&str>, key) {
            if let Ok(parsed) = T::from_str(value) {
                return parsed;
            }
        }

        // Try home config next
        if let Some(value) = self.global_config.get_from(None::<&str>, key) {
            if let Ok(parsed) = T::from_str(value) {
                return parsed;
            }
        }

        let value = self.def_config.get_from(None::<&str>, key).unwrap();
        let parsed = T::from_str(value).unwrap();
        return parsed;
    }

    pub fn set(&mut self, key: &str, value: &str) {
        self.user_config
            .set_to(None::<&str>, key.to_string(), value.to_string());
        self.user_config
            .write_to_file(&self.rc)
            .unwrap();
    }

    pub fn set_global(&mut self, key: &str, value: &str) {
        self.global_config
            .set_to(None::<&str>, key.to_string(), value.to_string());
        self.global_config
            .write_to_file(&self.global_nsvrc_path)
            .unwrap();
    }

    pub fn display(&self) -> String {
        let mut display = String::new();
        display.push_str(&format!("user_config: {:?}", self.user_config));
        display.push_str(&format!("global_config: {:?}", self.global_config));
        display
    }

    pub fn display_with_all(&self) -> String {
        let mut display = String::new();
        display.push_str(&format!("def_config: {:?}", self.def_config));
        display.push_str(&format!("user_config: {:?}", self.user_config));
        display.push_str(&format!("global_config: {:?}", self.global_config));
        display
    }
}
