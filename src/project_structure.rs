use std::fs;
use std::process::exit;

use serde::{Deserialize, Serialize};

use crate::PROJECT_NAME;

#[derive(Deserialize, Serialize, Default, Debug, Clone)]
pub struct Folder {
    pub name:        String,
    pub files:       Vec<File>,
    pub sub_folders: Vec<Folder>,
    pub commands:    Vec<String>,
}

#[derive(Deserialize, Serialize, Default, Debug, Clone)]
pub struct File {
    pub filename:      String,
    pub lines_of_file: Vec<String>,
}

#[derive(Deserialize, Serialize, Default, Debug)]
pub struct Template {
    pub folders:        Option<Vec<Folder>>,
    pub files:          Option<Vec<File>>,
    pub commands:       Option<Vec<String>>,
    pub folder_pointer: Option<String>,
    pub file_pointer:   Option<String>,
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

    project.sub_folders.extend(template.folders.unwrap_or_default());
    project.files.extend(template.files.unwrap_or_default());
    project.commands.extend(template.commands.unwrap_or_default());

    // open and parse folder pointer
    if let Some(_folder_pointer) = template.folder_pointer {
        todo!()
    }

    // open and parse file pointer
    if let Some(_file_pointer) = template.file_pointer {
        todo!()
    }
}
