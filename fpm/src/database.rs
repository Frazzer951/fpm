// JSON Databases

// Project Database
//  Load
//  Save

use crate::{config::Config, error::Error, project::Project, utils::Result};
use fs_err as fs;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
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

    pub fn load_database(config: Config) -> Result<Self> {
        let path = PathBuf::from(config.database_path);

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

    pub fn save_database(&self, config: Config) -> Result<()> {
        let path = PathBuf::from(config.database_path);

        let data = match serde_json::to_string(self) {
            Ok(d) => d,
            Err(e) => {
                return Err(Error::Json(e));
            },
        };

        match fs::write(path, data) {
            Ok(_) => Ok(()),
            Err(e) => Err(Error::IO(e)),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::test_funcs::is_same_file;
    use anyhow::Result;
    use std::path::Path;

    // TODO: Rewrite to actually test class

    #[test]
    fn internal() -> Result<()> {
        assert!(is_same_file(Path::new("./src/database.rs"), Path::new("./src/database.rs"))?);
        Ok(())
    }
}
