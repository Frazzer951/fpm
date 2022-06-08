use std::fs;
use std::io::Write;
use std::process::{exit, Command};

use serde::{Deserialize, Serialize};

use crate::PROJECT_NAME;

#[derive(Deserialize, Serialize, Default, Debug, Clone)]
pub struct Folder {
    pub name:        String,
    #[serde(default)]
    pub files:       Vec<File>,
    #[serde(default)]
    pub sub_folders: Vec<Folder>,
    #[serde(default)]
    pub commands:    Vec<String>,
}

#[derive(Deserialize, Serialize, Default, Debug, Clone)]
pub struct File {
    pub filename:      String,
    pub lines_of_file: Vec<String>,
}

#[derive(Deserialize, Serialize, Default, Debug)]
pub struct Template {
    #[serde(default)]
    pub folders:        Vec<Folder>,
    #[serde(default)]
    pub files:          Vec<File>,
    #[serde(default)]
    pub commands:       Vec<String>,
    pub folder_pointer: Option<String>,
    pub file_pointer:   Option<String>,
}

pub struct TemplateVars {
    pub project_name: String,
}

pub fn load_template(project: &mut Folder, mut template_name: String) {
    let mut template_dir = dirs::config_dir().unwrap();
    template_dir.push(PROJECT_NAME);
    template_dir.push("templates");

    // make sure path exists
    fs::create_dir_all(template_dir.clone()).unwrap();

    template_name.push_str(".yaml");
    template_dir.push(template_name.clone());

    let contents = match fs::read_to_string(template_dir) {
        Ok(c) => c,
        Err(_) => {
            eprintln!("The template: {} could not be found", template_name);
            exit(1);
        },
    };

    let template: Template = match serde_yaml::from_str(&contents) {
        Ok(d) => d,
        Err(_) => {
            eprintln!(
                "The template: {} could not be parsed. Please check it for errors",
                template_name
            );
            exit(1);
        },
    };

    project.sub_folders.extend(template.folders);
    project.files.extend(template.files);
    project.commands.extend(template.commands);

    // open and parse folder pointer
    if let Some(_folder_pointer) = template.folder_pointer {
        todo!()
    }

    // open and parse file pointer
    if let Some(_file_pointer) = template.file_pointer {
        todo!()
    }
}

pub fn build_folder(path: std::path::PathBuf, folder: &Folder, template_vars: &TemplateVars) {
    for sub_folder in folder.sub_folders.clone() {
        let mut f_path = path.clone();
        f_path.push(sub_folder.name.clone());
        fs::create_dir_all(f_path.clone())
            .unwrap_or_else(|_| panic!("Failed to create the directory {}", f_path.display()));
        build_folder(f_path, &sub_folder, template_vars)
    }

    for file in folder.files.clone() {
        build_file(path.clone(), &file, template_vars);
    }

    for command in folder.commands.clone() {
        let command_parts: Vec<&str> = command.split(' ').collect();
        // run the command stored in the command variable
        let mut cmd = Command::new(command_parts[0]);
        cmd.args(&command_parts[1..]);
        cmd.current_dir(path.clone());
        cmd.status()
            .unwrap_or_else(|err| panic!("Failed to run the command {} -- {}", command, err));
    }
}

pub fn build_file(mut path: std::path::PathBuf, file: &File, template_vars: &TemplateVars) {
    path.push(file.filename.clone());
    let mut f =
        fs::File::create(path).unwrap_or_else(|err| panic!("Failed to create the file {} -- {}", file.filename, err));
    for line in &file.lines_of_file {
        let line = process_template_vars(line, template_vars) + "\n";
        f.write_all(line.as_bytes())
            .unwrap_or_else(|err| panic!("Failed to write to file {} -- {}", file.filename, err));
    }
}

fn process_template_vars(string: &str, vars: &TemplateVars) -> String {
    let line = string.replace("{fpm_project_name}", vars.project_name.as_str());

    line
}
