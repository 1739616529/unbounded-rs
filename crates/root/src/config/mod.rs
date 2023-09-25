use std::{env, path::PathBuf};

use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string};
use tokio::fs::{metadata, read_to_string, write};

use crate::util::{get_home_dir, unwrap_join_all};

#[derive(Deserialize, Serialize, Clone)]
pub struct Config {
    pub(crate) port: u16,
}

pub static mut CONFIG: Option<Config> = None;

pub fn set(config: Config) {
    unsafe { CONFIG = Some(config) }

    // std::env::var("home");
}

pub async fn get() -> Config {
    // 如果已存在
    if let Some(config) = unsafe { CONFIG.clone() } {
        return config;
    }

    let config_path = get_config_path();

    let content = read_to_string(config_path).await.unwrap();

    let config_: Config = from_str(&content).unwrap();

    set(config_.clone());

    config_
}

pub async fn init(config: Config) {
    let config_path = get_config_path();

    let is_exist = metadata(config_path).await;

    if let Ok(_) = is_exist {
        return;
    }

    set(config);
    sync().await;
}

pub async fn sync() {
    if let Some(config) = unsafe { CONFIG.clone() } {
        let config_content = to_string(&config).unwrap();
        let config_path = get_config_path();
        write(config_path, config_content).await.unwrap();
    }
}

fn get_config_path() -> PathBuf {
    let home_dir = get_home_dir().unwrap();

    let mut config_path = PathBuf::new();

    config_path.push(home_dir);
    config_path.push(".unrc");

    config_path
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_home_dir() {
        let config_path = get_config_path();
        let mut config_path_2 = PathBuf::from(get_home_dir().unwrap());
        config_path_2.push(".unrc");
        assert!(config_path == config_path_2)
    }
}
