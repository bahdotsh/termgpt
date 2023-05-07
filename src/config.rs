use directories::ProjectDirs;
use serde::Deserialize;
use std::fs::{self, File};

#[derive(Deserialize, Debug)]
pub struct Config {
    pub token: String,
}

impl Config {
    pub fn make_config() {
        if let Some(dir) = ProjectDirs::from("", "", "rgpt") {
            let config_dir = dir.config_dir();
            if !config_dir.exists() {
                fs::create_dir(config_dir).unwrap();

                let file_path = config_dir.join("config.toml");
                File::create(file_path).unwrap();
            }
        }
    }
    pub fn get_config() {
        if let Some(dir) = ProjectDirs::from("", "", ".rgpt") {
            let config_dir = dir.config_dir();

            let config_file = fs::read_to_string(config_dir.join("config.toml")).unwrap();

            let _config: Config = toml::from_str(&config_file).unwrap();
        }
    }

    pub fn open_config_dir() {
        if let Some(dir) = ProjectDirs::from("", "", ".rgpt") {
            let config_dir = dir.config_dir();

            let config_file = fs::read_to_string(config_dir.join("config.toml")).unwrap();

            let _config: Config = toml::from_str(&config_file).unwrap();
        }
    }
}
