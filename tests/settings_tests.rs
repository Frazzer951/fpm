use std::env;

use fpm::settings::*;
use fpm::{CONFIG_FILENAME, PROJECT_ENV_PREFIX, PROJECT_NAME};
use fs_err as fs;
use serial_test::serial;

mod common;

#[test]
fn verify_default_settings() {
    assert_eq!(
        Settings::default(),
        Settings {
            base_dir:     None,
            template_dir: None,
            git_command:  "git clone {FPM_GIT_URL}".to_string(),
            config_dir:   "".to_string(),
        }
    )
}

#[test]
#[serial]
fn test_settings_new() {
    let mut test_dir = common::get_test_dir();
    test_dir.push("config.toml");
    env::set_var("FPM_CONFIG_DIR", test_dir.clone());

    // delete config.toml if it exists
    if test_dir.exists() {
        fs::remove_file(&test_dir).unwrap();
    }

    let settings = Settings::new();

    println!("Settings: {:#?}", settings);

    // delete config.toml if it exists
    if test_dir.exists() {
        fs::remove_file(test_dir).unwrap();
    }
}
