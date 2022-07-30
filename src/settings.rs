use std::env;
use std::path::PathBuf;

use config::{Config, Environment, File};
use fs_err as fs;
use serde::{Deserialize, Serialize};

use crate::{CONFIG_FILENAME, PROJECT_ENV_PREFIX, PROJECT_NAME};

#[derive(Deserialize, Serialize, Debug)]
pub struct Settings {
    #[serde(default)]
    pub base_dir:     Option<String>,
    #[serde(default)]
    pub template_dir: Option<String>,
    #[serde(default = "default_git_command")]
    pub git_command:  String,
    #[serde(skip_serializing)]
    pub config_dir:   String,
}

fn default_git_command() -> String { "git clone {FPM_GIT_URL}".to_string() }

impl Default for Settings {
    fn default() -> Self {
        Settings {
            base_dir:     None,
            template_dir: None,
            git_command:  default_git_command(),
            config_dir:   String::from(""),
        }
    }
}

impl Settings {
    pub fn new() -> Self {
        let config_dir = env::var(format!("{}_CONFIG_DIR", PROJECT_ENV_PREFIX)).unwrap_or_else(|_| {
            let mut config_dir = dirs::config_dir().unwrap();
            config_dir.push(PROJECT_NAME);
            config_dir.push(CONFIG_FILENAME);
            String::from(config_dir.to_str().unwrap())
        });

        // Create config file if it doesn't exist
        if !PathBuf::from(config_dir.clone()).exists() {
            let mut parent_dir = PathBuf::from(config_dir.clone());
            parent_dir.pop();
            fs::create_dir_all(&parent_dir).unwrap();
            let settings = Settings {
                config_dir: config_dir.clone(),
                ..Default::default()
            };
            settings.save();
        }

        let s = Config::builder()
            .add_source(File::with_name(config_dir.as_str()))
            .add_source(Environment::with_prefix(PROJECT_ENV_PREFIX))
            .set_override("config_dir", config_dir)
            .expect("Failed to set config_dir")
            .build()
            .expect("Failed to load and parse config file");

        s.try_deserialize().expect("Failed to Deserialize config file")
    }

    pub fn save(&self) {
        let config_path = PathBuf::from(self.config_dir.clone());

        // make sure path exists
        let mut config_dir = config_path.clone();
        config_dir.pop();
        fs::create_dir_all(config_dir.clone()).unwrap();

        // save config to config_dir
        let contents = toml::to_string(self).unwrap();
        fs::write(config_path, contents).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_default_git() { assert_eq!(default_git_command(), "git clone {FPM_GIT_URL}".to_string()) }
}
