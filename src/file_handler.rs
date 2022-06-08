use std::{fmt, fs};

use clap::ArgEnum;
use itertools::Itertools;
use serde::{Deserialize, Serialize};

use crate::{CONFIG_FILENAME, PROJECT_DB_FILENAME, PROJECT_NAME};

// region -- Custom Errors
type Result<T> = std::result::Result<T, FileError>;

#[derive(Debug, Clone)]
pub enum FileError {
    LoadingError,
    ParsingError,
}

impl fmt::Display for FileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FileError::LoadingError => {
                write!(f, "Failed to load config file")
            },
            FileError::ParsingError => {
                write!(f, "Failed to parse config file")
            },
        }
    }
}
// endregion

// region -- Config Struct
#[derive(Deserialize, Serialize, Default, Debug)]
pub struct Config {
    #[serde(default)]
    pub base_dir: Option<String>,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ArgEnum)]
pub enum ConfigOptions {
    BaseDir,
}
// endregion

// region -- Project Struct
#[derive(Deserialize, Serialize, Default, Debug, Clone, Eq, Hash, PartialEq)]
pub struct Project {
    pub name:      String,
    pub directory: String,
    #[serde(default)]
    pub category:  Option<String>,
    #[serde(default)]
    pub p_type:    Option<String>,
}
// endregion

pub fn load_config() -> Result<Config> {
    let mut config_dir = dirs::config_dir().unwrap();
    config_dir.push(PROJECT_NAME);
    config_dir.push(CONFIG_FILENAME);

    let contents = match fs::read_to_string(config_dir) {
        Ok(c) => Ok(c),
        Err(_) => Err(FileError::LoadingError),
    }?;

    match toml::from_str(&contents) {
        Ok(d) => Ok(d),
        Err(_) => Err(FileError::ParsingError),
    }
}

pub fn load_projects() -> Result<Vec<Project>> {
    let mut projects_dir = dirs::config_dir().unwrap();
    projects_dir.push(PROJECT_NAME);
    projects_dir.push(PROJECT_DB_FILENAME);

    let contents = match fs::read_to_string(projects_dir) {
        Ok(c) => Ok(c),
        Err(_) => Err(FileError::LoadingError),
    }?;

    match serde_json::from_str(&contents) {
        Ok(d) => Ok(d),
        Err(_) => Err(FileError::ParsingError),
    }
}

pub fn save_config(config: Config) {
    let mut config_dir = dirs::config_dir().unwrap();
    config_dir.push(PROJECT_NAME);

    // make sure path exists
    fs::create_dir_all(config_dir.clone()).unwrap();

    config_dir.push(CONFIG_FILENAME);

    // save config to config_dir
    let contents = toml::to_string(&config).unwrap();
    fs::write(config_dir, contents).unwrap();
}

pub fn save_projects(projects: Vec<Project>) {
    let mut projects_dir = dirs::config_dir().unwrap();
    projects_dir.push(PROJECT_NAME);

    // make sure path exists
    fs::create_dir_all(projects_dir.clone()).unwrap();

    projects_dir.push(PROJECT_DB_FILENAME);

    // remove duplicates
    let projects: Vec<Project> = projects.into_iter().unique().collect();

    // save config to config_dir
    let contents = serde_json::to_string(&projects).unwrap();
    fs::write(projects_dir, contents).unwrap();
}
