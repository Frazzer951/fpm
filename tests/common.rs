use std::env;
use std::path::PathBuf;

pub fn get_test_dir() -> PathBuf {
    let mut dir = env::current_dir().unwrap();
    dir.push("tests");
    dir
}
