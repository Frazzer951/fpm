use crate::utils::config_folder;

#[derive(Debug)]
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
        db_path.push("fpm");
        db_path.push("projectDB.db");

        Self {
            database_path: db_path.to_str().unwrap_or_default().to_owned(),
        }
    }
}
