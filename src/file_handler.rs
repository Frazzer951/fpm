use std::path::PathBuf;

use fs_err as fs;
use itertools::Itertools;
use serde::{Deserialize, Serialize};

use crate::{PROJECT_DB_FILENAME, PROJECT_NAME};

type Result<T> = std::result::Result<T, FileError>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FileError {
    LoadingError,
    ParsingError,
}

#[derive(PartialOrd, Ord, Deserialize, Serialize, Default, Debug, Clone, Eq, Hash, PartialEq)]
pub struct Project {
    pub name:      String,
    pub directory: String,
    #[serde(default)]
    pub category:  Option<String>,
    #[serde(default)]
    pub p_type:    Option<String>,
}

#[derive(Default, Clone, Eq, PartialEq, Debug)]
pub struct Projects {
    pub projects:     Vec<Project>,
    pub project_path: PathBuf,
}

impl Projects {
    pub fn load(projects_dir: PathBuf) -> Result<Self> {
        // Load the file
        let contents = match fs::read_to_string(&projects_dir) {
            Ok(c) => Ok(c),
            Err(_) => Err(FileError::LoadingError),
        }?;

        // Parse the file
        match serde_json::from_str(&contents) {
            Ok(d) => Ok(Projects {
                projects:     d,
                project_path: projects_dir,
            }),
            Err(_) => Err(FileError::ParsingError),
        }
    }

    pub fn save(&self) {
        // Get the directory of the project database file
        let projects_path = self.project_path.clone();
        let mut projects_dir = self.project_path.clone();
        projects_dir.pop();

        // make sure path exists
        fs::create_dir_all(projects_dir).unwrap();

        // remove duplicates
        let projects: Vec<&Project> = self.projects.iter().unique().sorted().collect();

        // save config to config_dir
        let contents = serde_json::to_string(&projects).unwrap();
        fs::write(projects_path, contents).unwrap();
    }

    pub fn default_dir() -> PathBuf {
        let mut projects_dir = dirs::config_dir().unwrap();
        projects_dir.push(PROJECT_NAME);
        projects_dir.push(PROJECT_DB_FILENAME);
        projects_dir
    }
}
