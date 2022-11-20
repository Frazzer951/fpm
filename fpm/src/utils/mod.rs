use std::path::PathBuf;

use dirs::config_dir;

use crate::error::Error;

pub type Result<T> = core::result::Result<T, Error>;

pub fn config_folder() -> PathBuf {
    config_dir().unwrap_or_default()
}
