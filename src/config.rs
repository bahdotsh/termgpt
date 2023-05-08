use directories::ProjectDirs;
use serde::Deserialize;
use std::any::Any;
use std::fs::{self, File};
use std::io::{self, Write};
use toml_edit::{value, Document};
use std::process;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub api: String,
}

impl Config {
    pub fn make_config() {
        if let Some(dir) = ProjectDirs::from("", "", "termgpt") {
            let config_dir = dir.config_dir();
            if !config_dir.exists() {
                fs::create_dir(config_dir).unwrap();

                let file_path = config_dir.join("config.toml");
                File::create(file_path).unwrap();
            }
        }
    }

    pub fn set_config<T: Any + Into<toml_edit::Value>>(
        name: &str,
        new_content: T,
    ) -> Result<(), std::io::Error> {
        let dir = ProjectDirs::from("", "", "termgpt").unwrap();
        let config_dir = dir.config_dir();
        if config_dir.exists() {
            let config_file = fs::read_to_string(config_dir.join("config.toml"))?;
            let mut config_write = fs::OpenOptions::new()
                .write(true)
                .truncate(true)
                .open(config_dir.join("config.toml"))?;
            let mut doc = config_file.parse::<Document>().expect("invalid document");
            doc[name] = value(new_content);
            write!(config_write, "{}", doc.to_string())?;

            Ok(())
        } else {
            return Err(io::Error::new(io::ErrorKind::Other, "An error occurred."));
        }
    }
    pub fn get_api_config() -> String {
        let dir = ProjectDirs::from("", "", "termgpt").unwrap_or_else(|| {
            eprintln!("Failed to get configuration directory");
            process::exit(1);
        });
        let config_dir = dir.config_dir();

        let config_file = fs::read_to_string(config_dir.join("config.toml")).unwrap_or_else(|err| {
            eprintln!("Unable to open config file: {}", err);
            process::exit(1);
        });

        let config: Config = toml::from_str(&config_file).unwrap_or_else(|err| {
            eprintln!("Unable to parse config file: {}", err);
            process::exit(1);
        });

        config.api
    }

}
