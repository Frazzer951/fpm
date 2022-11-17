use std::path::PathBuf;

use dirs::config_dir;

pub fn config_folder() -> Option<PathBuf> {
    config_dir()
}
