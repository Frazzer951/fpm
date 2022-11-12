use std::env;

use fpm::settings::*;
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
            database_dir: None,
            git_command:  "git clone {fpm_git_url} --recursive {fpm_project_dir}".to_string(),
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

    assert_eq!(
        settings,
        Settings {
            base_dir:     None,
            template_dir: None,
            database_dir: None,
            git_command:  "git clone {fpm_git_url} --recursive {fpm_project_dir}".to_string(),
            config_dir:   test_dir.to_str().unwrap().to_string(),
        }
    );

    // delete config.toml if it exists
    if test_dir.exists() {
        fs::remove_file(test_dir).unwrap();
    }
}

#[test]
#[serial]
fn test_settings_new_from_existing() {
    let mut test_dir = common::get_test_dir();
    test_dir.push("example_files");
    test_dir.push("config.toml");
    env::set_var("FPM_CONFIG_DIR", test_dir.clone());

    let settings = Settings::new();

    assert_eq!(
        settings,
        Settings {
            base_dir:     Some("/home/frazzer/dev".to_string()),
            template_dir: Some("/home/frazzer/dev/templates".to_string()),
            database_dir: Some("/home/frazzer/dev/database".to_string()),
            git_command:  "git clone {FPM_GIT_URL}".to_string(),
            config_dir:   test_dir.to_str().unwrap().to_string(),
        }
    );
}
