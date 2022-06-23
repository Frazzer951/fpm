use std::io;
use std::path::{Path, PathBuf};

use fs_err as fs;
use walkdir::WalkDir;

pub fn move_folder<P, Q>(from: P, to: Q, print_progress: bool) -> io::Result<u64>
where
    P: AsRef<Path>,
    Q: AsRef<Path>,
{
    copy_folder(from.as_ref(), to, print_progress)?;
    fs::remove_dir_all(from).unwrap();
    Ok(0)
}

pub fn copy_folder<P, Q>(from: P, to: Q, print_progress: bool) -> io::Result<u64>
where
    P: AsRef<Path>,
    Q: AsRef<Path>,
{
    let from = from.as_ref();
    let to = to.as_ref();
    for entry in WalkDir::new(from) {
        let entry = entry?;
        if entry.file_type().is_dir() {
            let to_path = replace_prefix(entry.path(), from, to);
            fs::create_dir_all(&to_path)?;
        } else {
            let to_file = replace_prefix(entry.path(), from, to);
            if print_progress {
                println!("Moving {:?} to {}", entry.file_name(), to_file.display());
            }
            fs::copy(entry.path(), &to_file)?;
        }
    }
    Ok(0)
}

fn replace_prefix<P, Q, R>(path: P, from_prefix: Q, to_prefix: R) -> PathBuf
where
    P: AsRef<Path>,
    Q: AsRef<Path>,
    R: AsRef<Path>,
{
    let relative = path.as_ref().strip_prefix(from_prefix).unwrap();
    let new_path = to_prefix.as_ref().join(relative);

    new_path
}
