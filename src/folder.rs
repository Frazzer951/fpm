use std::io::Write;

use crate::template::{File, Folder};

/// Take in a folder struct and a base path and create all the folders and files it contains
pub fn process_folder(path: std::path::PathBuf, folder: &Folder, proj_name: &str) -> std::io::Result<()> {
    if folder.sub_folders.is_some() {
        for f in folder.sub_folders.as_ref().unwrap() {
            let mut new_path = path.clone();
            new_path.push(f.name.clone());
            std::fs::create_dir_all(&new_path)?;
            process_folder(new_path, f, proj_name)?;
        }
    }
    for f in &folder.files {
        create_file(path.clone(), f, proj_name)?;
    }
    Ok(())
}

/// Create a file from a file struct and replace any <name> tags with proj_name
pub fn create_file(mut path: std::path::PathBuf, file: &File, proj_name: &str) -> std::io::Result<()> {
    path.push(file.name.clone());
    let mut f = std::fs::File::create(path)?;
    for line in &file.lines {
        let line = line.replace("<name>", proj_name) + "\n";
        f.write_all(line.as_bytes())?;
    }

    Ok(())
}
