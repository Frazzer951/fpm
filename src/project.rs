use std::collections::HashMap;
use std::path::PathBuf;
use std::process::{exit, Command};
use std::{fs, io};

use clap::ArgMatches;
use strsim::osa_distance;

use crate::{build_folder, load_template, save_projects, Folder, Project, Settings, TemplateVars};

pub fn project_handler(projects: &mut Vec<Project>, project_name: String, command: Option<(&str, &ArgMatches)>) {
    match command {
        Some(("verify", _sub_matches)) => {
            verify_projects(projects.clone(), project_name);
        },
        _ => unreachable!(),
    }
}

pub fn add_project(
    mut projects: Vec<Project>,
    name: String,
    directory: String,
    p_type: Option<String>,
    category: Option<String>,
) {
    // is there a folder at directory?
    if !std::path::Path::new(&directory).exists() {
        eprintln!("The directory `{}` specified does not exist", directory);
        exit(1);
    }

    // add project to known projects
    projects.push(Project {
        name,
        directory,
        category,
        p_type,
    });
    save_projects(projects);
}

pub fn new_project(
    mut settings: &mut Settings,
    mut projects: Vec<Project>,
    project_vars: Project,
    git_url: Option<String>,
    templates: Vec<String>,
) {
    let dir = project_vars.directory;
    let mut project_path = std::path::PathBuf::from(dir.clone());
    if project_vars.category.is_some() {
        project_path.push(project_vars.category.as_ref().unwrap());
    }
    if project_vars.p_type.is_some() {
        project_path.push(project_vars.p_type.as_ref().unwrap());
    }
    project_path.push(project_vars.name.clone());

    let mut project = Folder {
        name:        project_vars.name.clone(),
        files:       vec![],
        sub_folders: vec![],
        commands:    vec![],
    };

    if settings.template_dir.is_none() {
        let mut template_path = PathBuf::from(dir);
        template_path.push("templates");
        settings.template_dir = Some(String::from(template_path.to_str().unwrap()));
    }

    for template in templates {
        load_template(settings, &mut project, template.clone());
    }

    // create project folders
    fs::create_dir_all(project_path.clone()).unwrap();

    // if the folder at project_path isn't empty throw an error
    if fs::read_dir(project_path.clone()).unwrap().count() > 0 {
        eprintln!("The directory specified already exists and is not empty");
        eprintln!("{:#?}", project_path);
        return;
    }

    let template_vars = TemplateVars {
        project_name: project_vars.name.clone(),
    };

    // build the project
    build_folder(project_path.clone(), &project, &template_vars);

    // run git clone command
    if let Some(git_url) = git_url {
        let command = settings.git_command.replace("{FPM_GIT_URL}", git_url.as_str());

        let command_parts: Vec<&str> = command.split(' ').collect();
        // run the command stored in the command variable
        let mut cmd = Command::new(command_parts[0]);
        cmd.args(&command_parts[1..]);
        cmd.current_dir(project_path.clone());
        cmd.status()
            .unwrap_or_else(|err| panic!("Failed to run the command {} -- {}", command, err));
    }

    // add project to known projects
    projects.push(Project {
        name:      project_vars.name,
        directory: String::from(project_path.to_str().unwrap()),
        category:  project_vars.category.clone(),
        p_type:    project_vars.p_type.clone(),
    });
    save_projects(projects);
}

pub fn verify_projects(mut projects: Vec<Project>, name: String) {
    let mut projects_to_remove = vec![];

    for mut project in &mut projects {
        if project.name == name || name == "*" {
            // check if the folder stored in directory exits
            if !std::path::Path::new(&project.directory).exists() {
                println!(
                    "{} - The directory `{}` does not exist",
                    project.name, project.directory
                );
                // ask if the user wishes to modify this project
                let mut input = String::new();
                println!("Do you wish to modify this project? (y/n/r)");
                io::stdin().read_line(&mut input).unwrap();
                if input.trim() == "y" {
                    // ask for the new directory
                    println!("Enter the new directory");
                    io::stdin().read_line(&mut input).unwrap();
                    project.directory = input.trim().to_string();
                } else if input.trim() == "r" {
                    println!("Are you sure you want to remove this projects? (y/n)");
                    input = String::new();
                    io::stdin().read_line(&mut input).unwrap();
                    if input.trim() == "y" {
                        // remove project from projects
                        projects_to_remove.push(project.clone());
                    }
                }
            }
        }
    }

    projects.retain(|proj| {
        for p in &projects_to_remove {
            if proj.name == p.name && proj.directory == p.directory {
                return false;
            }
        }
        true
    });
    save_projects(projects)
}

pub fn get_similar(projects: &[Project], name: &str) -> (usize, Vec<String>) {
    let names: Vec<String> = projects.iter().map(|proj| proj.name.clone()).collect();
    let names = names.iter().map(|s| s as &str).collect();
    let name_distances = similar_strings(name, names);
    let min = name_distances.iter().min_by_key(|entry| entry.0).unwrap();
    (*min.0, min.1.iter().map(|&s| s.into()).collect())
}

fn similar_strings<'a>(input: &'a str, strings: Vec<&'a str>) -> HashMap<usize, Vec<&'a str>> {
    let mut distances: HashMap<usize, Vec<&str>> = HashMap::new();

    for str in strings {
        let dist = osa_distance(input, str);
        distances.entry(dist).or_default().push(str);
    }

    distances
}
