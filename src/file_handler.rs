use std::fmt;

use fs_err as fs;
use itertools::Itertools;
use serde::{Deserialize, Serialize};

use crate::{PROJECT_DB_FILENAME, PROJECT_NAME};

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

// region -- Project Struct
#[derive(PartialOrd, Ord, Deserialize, Serialize, Default, Debug, Clone, Eq, Hash, PartialEq)]
pub struct Project {
    pub name:      String,
    pub directory: String,
    #[serde(default)]
    pub category:  Option<String>,
    #[serde(default)]
    pub p_type:    Option<String>,
}
// endregion

pub fn load_projects() -> Result<Vec<Project>> {
    // Get the directory of the project database file
    let mut projects_dir = dirs::config_dir().unwrap();
    projects_dir.push(PROJECT_NAME);
    projects_dir.push(PROJECT_DB_FILENAME);

    // Load the file
    let contents = match fs::read_to_string(projects_dir) {
        Ok(c) => Ok(c),
        Err(_) => Err(FileError::LoadingError),
    }?;

    // Parse the file
    match serde_json::from_str(&contents) {
        Ok(d) => Ok(d),
        Err(_) => Err(FileError::ParsingError),
    }
}

pub fn save_projects(projects: &[Project]) {
    // Get the directory of the project database file
    let mut projects_dir = dirs::config_dir().unwrap();
    projects_dir.push(PROJECT_NAME);

    // make sure path exists
    fs::create_dir_all(projects_dir.clone()).unwrap();

    projects_dir.push(PROJECT_DB_FILENAME);

    // remove duplicates
    let projects: Vec<&Project> = projects.iter().unique().sorted().collect();

    // save config to config_dir
    let contents = serde_json::to_string(&projects).unwrap();
    fs::write(projects_dir, contents).unwrap();
}
