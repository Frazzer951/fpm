// JSON Databases

// Project Database
//  Load
//  Save

use crate::{config::Config, error::Error, project::Project, utils::Result};
use fs_err as fs;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Database {
    projects: Vec<Project>,
}

impl Default for Database {
    fn default() -> Self {
        Self::new()
    }
}

impl Database {
    pub fn new() -> Self {
        Self { projects: vec![] }
    }

    pub fn load_database(config: &Config) -> Result<Self> {
        let path = PathBuf::from(&config.database_path);

        let data = match fs::read_to_string(path) {
            Ok(d) => d,
            Err(e) => {
                return Err(Error::IO(e));
            },
        };
        let db = match serde_json::from_str::<Database>(&data) {
            Ok(db) => db,
            Err(e) => {
                return Err(Error::Json(e));
            },
        };

        Ok(db)
    }

    pub fn save_database(&self, config: &Config) -> Result<()> {
        let path = PathBuf::from(&config.database_path);

        let data = match serde_json::to_string(self) {
            Ok(d) => d,
            Err(e) => {
                return Err(Error::Json(e));
            },
        };

        let parent_dir = path.parent();
        if let Some(parent_dir) = parent_dir {
            match fs::create_dir_all(parent_dir) {
                Ok(_) => {},
                Err(e) => {
                    return Err(Error::IO(e));
                },
            }
        }

        match fs::write(path, data) {
            Ok(_) => Ok(()),
            Err(e) => Err(Error::IO(e)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{config::Config, utils::test_utils::is_same_file};
    use anyhow::Result;
    use std::path::Path;

    // TODO: Write tests for database with actual projects

    #[test]
    fn test_load_empty_database() -> Result<()> {
        let config = Config {
            database_path: "./tests/expected_files/emptyDB.json".to_owned(),
        };

        let db = Database::load_database(&config)?;

        assert_eq!(Database::default(), db);

        Ok(())
    }

    #[test]
    fn test_save_empty_database() -> Result<()> {
        let db = Database::default();
        let config = Config {
            database_path: "./tests/test_files/emptyDB.json".to_owned(),
        };

        if Path::new(&config.database_path).exists() {
            fs::remove_file(&config.database_path)?;
        }

        db.save_database(&config)?;

        assert!(is_same_file(
            Path::new("./tests/expected_files/emptyDB.json"),
            Path::new("./tests/test_files/emptyDB.json")
        )?);

        Ok(())
    }
}
