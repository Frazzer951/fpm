use crate::error::Error;
use crate::utils::{config_folder, Result};
use fs_err as fs;
use serde::{Deserialize, Serialize};
use std::io::Write;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub database_path: String,
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

impl Config {
    pub fn new() -> Self {
        let cf = config_folder();

        let mut db_path = cf;
        db_path.push("projectDB.db");

        Self {
            database_path: db_path.to_str().unwrap_or_default().to_owned(),
        }
    }

    pub fn load() -> Result<Self> {
        let cf = config_folder();
        let mut config_path = cf;
        config_path.push("fpm.toml");

        let content = match fs::read_to_string(config_path) {
            Ok(c) => c,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => String::new(),
            Err(e) => return Err(Error::IO(e)),
        };

        if content.is_empty() {
            Ok(Config::default())
        } else {
            match toml::from_str::<Config>(&content) {
                Ok(c) => Ok(c),
                Err(e) => Err(Error::TomlDes(e)),
            }
        }
    }

    pub fn save(&self) -> Result<()> {
        let cf = config_folder();
        let mut config_path = cf.clone();
        config_path.push("fpm.toml");

        fs::create_dir_all(cf)?;

        let mut file = match fs::File::create(config_path) {
            Ok(f) => f,
            Err(e) => return Err(Error::IO(e)),
        };

        let serialized = match toml::to_string(self) {
            Ok(s) => s,
            Err(e) => return Err(Error::TomlSer(e)),
        };

        match file.write(serialized.as_bytes()) {
            Ok(_) => {},
            Err(e) => return Err(Error::IO(e)),
        };

        Ok(())
    }
}
