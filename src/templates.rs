use std::collections::{HashMap, HashSet};
use std::io::Write;
use std::path::PathBuf;
use std::process::{exit, Command};

use fs_err as fs;
use serde::Deserialize;

use crate::Settings;

#[derive(Debug)]
pub struct Templates {
    pub templates: HashMap<String, Template>,
    template_dir:  PathBuf,
}

impl Templates {
    pub fn new(settings: &Settings) -> Self {
        // Get the template directory
        let template_dir = PathBuf::from(settings.template_dir.clone().unwrap());

        // make sure path exists
        fs::create_dir_all(template_dir.clone()).unwrap();

        // Load the templates
        let mut templates = HashMap::new();
        for entry in fs::read_dir(template_dir.clone()).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_file() {
                let ext = path.extension().unwrap_or_default();
                if ext == "yaml" || ext == "yml" {
                    let template_name = path.file_stem().unwrap().to_str().unwrap().to_string();
                    let template = Template::load(path, template_name.clone());
                    templates.insert(template_name, template);
                }
            }
        }

        Templates {
            templates,
            template_dir,
        }
    }

    pub fn load_templates(&self, templates: Vec<String>, project: &mut Folder) -> Vec<String> {
        let mut added_templates = HashSet::new();
        self.load_templates_helper(templates, project, &mut added_templates)
    }

    pub fn load_templates_helper(
        &self,
        templates: Vec<String>,
        project: &mut Folder,
        added_templates: &mut HashSet<String>,
    ) -> Vec<String> {
        let mut template_vars = vec![];
        for template in templates {
            if !added_templates.contains(&template) {
                added_templates.insert(template.clone());
                // get the template from the templates map
                let template = self
                    .templates
                    .get(&template)
                    .unwrap_or_else(|| panic!("The template {} does not exist", template));

                // Add the template to the project
                project.sub_folders.extend(template.folders.clone());
                project.files.extend(template.files.clone());
                project.commands.extend(template.commands.clone());

                let template_dir = self.template_dir.clone();

                // open and parse folder pointer
                if let Some(folder_pointer) = template.folder_pointer.clone() {
                    let mut file_path = template_dir.clone();
                    file_path.push(folder_pointer);
                    let folder = load_folder(file_path);
                    project.sub_folders.extend(folder.sub_folders);
                    project.files.extend(folder.files);
                }

                // open and parse file pointer
                if let Some(file_pointer) = template.file_pointer.clone() {
                    let mut file_path = template_dir.clone();
                    file_path.push(file_pointer.file_dir);
                    let file = load_file(file_path);
                    if file_pointer.parent_folder.is_some() {
                        let mut parent_folder = file_pointer.parent_folder.unwrap();
                        parent_folder.add_file_to_deepest_folder(file);
                        project.sub_folders.push(parent_folder);
                    } else {
                        project.files.push(file);
                    }
                }

                if !template.include.is_empty() {
                    template_vars.extend(self.load_templates_helper(template.include.clone(), project, added_templates));
                }
                template_vars.extend(template.template_vars.clone());
            }
        }
        template_vars
    }
}

#[derive(Deserialize, Default, Debug)]
pub struct Template {
    #[serde(default)]
    pub folders:        Vec<Folder>,
    #[serde(default)]
    pub files:          Vec<File>,
    #[serde(default)]
    pub commands:       Vec<String>,
    #[serde(default)]
    pub template_vars:  Vec<String>,
    #[serde(default)]
    pub include:        Vec<String>,
    pub folder_pointer: Option<String>,
    pub file_pointer:   Option<FilePointer>,
}

impl Template {
    pub fn load(template_dir: PathBuf, template_name: String) -> Self {
        // Load the file
        let contents = match fs::read_to_string(&template_dir) {
            Ok(c) => c,
            Err(_) => {
                eprintln!("The template: {} could not be found", template_name);
                exit(1);
            },
        };

        // Parse the file
        let template: Template = match serde_yaml::from_str(&contents) {
            Ok(d) => d,
            Err(err) => {
                eprintln!(
                    "The template: {} could not be parsed. Please check it for errors\n{}",
                    template_name, err
                );
                exit(1);
            },
        };

        template
    }
}

#[derive(Deserialize, Default, Debug, Clone)]
pub struct Folder {
    pub name:        String,
    #[serde(default)]
    pub files:       Vec<File>,
    #[serde(default)]
    pub sub_folders: Vec<Folder>,
    #[serde(default)]
    pub commands:    Vec<String>,
}

impl Folder {
    pub fn add_file_to_deepest_folder(&mut self, file: File) {
        if self.sub_folders.is_empty() {
            self.files.push(file);
        } else {
            self.sub_folders[0].add_file_to_deepest_folder(file);
        }
    }
}

#[derive(Deserialize, Default, Debug, Clone)]
pub struct File {
    pub filename:      String,
    pub lines_of_file: Vec<String>,
}

#[derive(Deserialize, Default, Debug, Clone)]
pub struct FilePointer {
    pub file_dir:      String,
    pub parent_folder: Option<Folder>,
}

pub struct TemplateVars {
    pub project_name: String,
}

pub fn build_folder(
    path: std::path::PathBuf,
    folder: &Folder,
    template_vars: &TemplateVars,
    user_vars: &Vec<(String, String)>,
) {
    for sub_folder in folder.sub_folders.clone() {
        let mut f_path = path.clone();
        f_path.push(sub_folder.name.clone());
        fs::create_dir_all(f_path.clone())
            .unwrap_or_else(|_| panic!("Failed to create the directory {}", f_path.display()));
        build_folder(f_path, &sub_folder, template_vars, user_vars)
    }

    for file in folder.files.clone() {
        build_file(path.clone(), &file, template_vars, user_vars);
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

pub fn build_file(
    mut path: std::path::PathBuf,
    file: &File,
    template_vars: &TemplateVars,
    user_vars: &Vec<(String, String)>,
) {
    path.push(file.filename.clone());
    let mut f =
        fs::File::create(path).unwrap_or_else(|err| panic!("Failed to create the file {} -- {}", file.filename, err));
    for line in &file.lines_of_file {
        let line = process_template_vars(line, template_vars, user_vars) + "\n";
        f.write_all(line.as_bytes())
            .unwrap_or_else(|err| panic!("Failed to write to file {} -- {}", file.filename, err));
    }
}

fn process_template_vars(string: &str, vars: &TemplateVars, user_vars: &Vec<(String, String)>) -> String {
    let mut line = string.replace("{fpm_project_name}", vars.project_name.as_str());

    for (template, value) in user_vars {
        line = line.replace(format!("{{{}}}", template).as_str(), value.as_str());
    }

    line
}

fn load_file(path: PathBuf) -> File {
    let contents = match fs::read_to_string(path.clone()) {
        Ok(c) => c,
        Err(_) => {
            eprintln!("The file: {} could not be found", path.display());
            exit(1);
        },
    };

    File {
        filename:      path.file_name().unwrap().to_str().unwrap().to_string(),
        lines_of_file: contents.lines().map(|x| x.to_string()).collect(),
    }
}

fn load_folder(path: PathBuf) -> Folder {
    let mut folder = Folder {
        name:        path.file_name().unwrap().to_str().unwrap().to_string(),
        files:       vec![],
        sub_folders: vec![],
        commands:    vec![],
    };

    // walk the directory at path
    for entry in fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_dir() {
            folder.sub_folders.push(load_folder(path));
        } else {
            folder.files.push(load_file(path));
        }
    }

    folder
}
