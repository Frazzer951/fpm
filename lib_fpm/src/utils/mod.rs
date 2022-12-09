use std::path::PathBuf;

use dirs::config_dir;

use crate::error::Error;

pub type Result<T> = core::result::Result<T, Error>;

pub fn config_folder() -> PathBuf {
    config_dir().unwrap_or_default()
}

#[cfg(test)]
pub mod test_utils {
    use super::*;
    use fs_err as fs;
    use std::path::Path;

    pub fn is_same_file(file1: &Path, file2: &Path) -> Result<bool> {
        let f1 = match fs::read_to_string(file1) {
            Ok(d) => d,
            Err(e) => {
                return Err(Error::IO(e));
            },
        };
        let f2 = match fs::read_to_string(file2) {
            Ok(d) => d,
            Err(e) => {
                return Err(Error::IO(e));
            },
        };

        if f1 == f2 {
            return Ok(true);
        }

        for diff in diff::lines(&f1, &f2) {
            match diff {
                diff::Result::Left(l) => println!("-{}", l),
                diff::Result::Both(l, _) => println!(" {}", l),
                diff::Result::Right(r) => println!("+{}", r),
            }
        }
        Ok(false)
    }
}
