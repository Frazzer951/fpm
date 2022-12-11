use crate::error::Error;
use dirs::config_dir;
use std::path::PathBuf;

pub type Result<T> = core::result::Result<T, Error>;

pub fn config_folder() -> PathBuf {
    config_dir().unwrap_or_default()
}
