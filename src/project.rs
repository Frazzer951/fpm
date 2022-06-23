use std::collections::{HashMap, HashSet};
use std::io;
use std::path::PathBuf;
use std::process::{exit, Command};

use clap::ArgMatches;
use fs_err as fs;
use strsim::osa_distance;

use crate::utils::move_folder;
use crate::{build_folder, load_template, save_projects, Folder, Project, Settings, TemplateVars};

#[derive(Debug)]
pub struct VerifyFlags {
    list:           bool,
    remove_invalid: bool,
    interactive:    bool,
}

#[derive(Debug)]
pub struct RefactorFlags {
    dry_run:     bool,
    force:       bool,
    interactive: bool,
}

#[derive(Debug)]
pub struct EditOptions {
    name:            Option<String>,
    directory:       Option<String>,
    p_type:          Option<String>,
    category:        Option<String>,
    remove_type:     bool,
    remove_category: bool,
}

pub fn project_handler(
    projects: &mut Vec<Project>,
    project_name: String,
    settings: Settings,
    command: Option<(&str, &ArgMatches)>,
) {
    match command {
        Some(("verify", sub_matches)) => {
            let list = sub_matches.get_one::<bool>("list").cloned().expect("BOOL VALUE");
            let remove = sub_matches.get_one::<bool>("remove").cloned().expect("BOOL VALUE");
            let interactive = sub_matches.get_one::<bool>("interactive").cloned().expect("BOOL VALUE");

            let flags = VerifyFlags {
                list,
                remove_invalid: remove,
                interactive,
            };

            verify_projects(projects.to_owned(), project_name, flags);
        },
        Some(("refactor", sub_matches)) => {
            let dry_run = sub_matches.get_one::<bool>("dry-run").cloned().expect("BOOL VALUE");
            let force = sub_matches.get_one::<bool>("force").cloned().expect("BOOL VALUE");
            let interactive = sub_matches.get_one::<bool>("interactive").cloned().expect("BOOL VALUE");
            let directory = sub_matches.get_one::<String>("directory").cloned();

            if settings.base_dir.is_none() && directory.is_none() {
                eprintln!("No directory was specified, and the global Base Directory is not Set.");
                eprintln!("Specify a directory in the command, or Set a global directory with the config command`");
                return;
            }

            let dir = directory.unwrap_or_else(|| settings.base_dir.as_ref().unwrap().clone());

            refactor_projects(
                projects.to_owned(),
                project_name,
                dir,
                RefactorFlags {
                    dry_run,
                    force,
                    interactive,
                },
            );
        },
        Some(("edit", sub_matches)) => {
            let name = sub_matches.get_one::<String>("name").cloned();
            let directory = sub_matches.get_one::<String>("directory").cloned();
            let p_type = sub_matches.get_one::<String>("type").cloned();
            let category = sub_matches.get_one::<String>("category").cloned();
            let remove_type = sub_matches.get_one::<bool>("remove_type").cloned().expect("BOOL VALUE");
            let remove_category = sub_matches
                .get_one::<bool>("remove_category")
                .cloned()
                .expect("BOOL VALUE");

            if project_name == "*" {
                eprintln!("You must specify a project name to edit.");
                exit(1);
            }

            edit_project(
                projects.to_vec(),
                project_name,
                EditOptions {
                    name,
                    directory,
                    p_type,
                    category,
                    remove_type,
                    remove_category,
                },
            );
        },
        _ => unreachable!(),
    }
}

pub fn add_project(
    projects: &mut Vec<Project>,
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

pub fn add_project_from_folder(
    mut projects: Vec<Project>,
    path: PathBuf,
    p_type: Option<String>,
    category: Option<String>,
) {
    // is there a folder at path?
    if !std::path::Path::new(&path).exists() {
        eprintln!("The directory `{}` specified does not exist", path.display());
        exit(1);
    }

    for folder in fs::read_dir(path).unwrap() {
        let folder = folder.unwrap();
        let path = folder.path();
        if path.is_dir() {
            let name = path.file_name().unwrap().to_string_lossy().to_string();
            let directory = path.to_str().unwrap().to_string();

            // if there is a project with the same name, don't add it
            if projects.iter().any(|p| p.name == name || p.directory == directory) {
                continue;
            }

            println!("Would you like to add the folder `{}` ({})? (y/n):", name, directory);
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            if input.trim() == "y" {
                add_project(&mut projects, name, directory, p_type.clone(), category.clone());
            }
        }
    }
}

pub fn new_project(
    mut settings: &mut Settings,
    mut projects: Vec<Project>,
    project_vars: Project,
    git_url: Option<String>,
    templates: Vec<String>,
    open: bool,
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

    let mut user_template_vars = HashSet::new();

    for template in templates {
        user_template_vars.extend(load_template(settings, &mut project, template.clone()).into_iter());
    }

    // create project folders
    fs::create_dir_all(project_path.clone()).unwrap();

    // if the folder at project_path isn't empty throw an error
    if fs::read_dir(project_path.clone()).unwrap().count() > 0 {
        eprintln!("The directory specified already exists and is not empty");
        eprintln!("{:#?}", project_path);
        return;
    }

    let user_template_vars = user_template_vars
        .into_iter()
        .map(|f| {
            // get user input for template variables
            println!("Enter the value for {}: ", f);
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            (f, input.trim().to_string())
        })
        .collect::<Vec<(String, String)>>();

    let template_vars = TemplateVars {
        project_name: project_vars.name.clone(),
    };

    // build the project
    build_folder(project_path.clone(), &project, &template_vars, &user_template_vars);

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
    save_projects(&projects);

    if open {
        // open dir
        opener::open(project_path).expect("Failed to open the directory");
    }
}

pub fn verify_projects(mut projects: Vec<Project>, name: String, flags: VerifyFlags) {
    let mut projects_to_remove = vec![];

    for mut project in &mut projects {
        if project.name == name || name == "*" {
            // check if the folder stored in directory exits
            if !std::path::Path::new(&project.directory).exists() {
                println!(
                    "{} - The directory `{}` does not exist",
                    project.name, project.directory
                );
                if !flags.list {
                    if flags.remove_invalid {
                        projects_to_remove.push(project.clone());
                    } else if flags.interactive {
                        // ask if the user wishes to modify this project
                        let mut input = String::new();
                        println!("Do you wish to modify this project? (y/n/r)");
                        io::stdin().read_line(&mut input).unwrap();
                        if input.trim() == "y" {
                            // ask for the new directory
                            println!("Enter the new directory");
                            input = String::new();
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
                    } else {
                        panic!("How did you get here?");
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
    save_projects(&projects)
}

pub fn refactor_projects(mut projects: Vec<Project>, name: String, base_dir: String, flags: RefactorFlags) {
    for project in &mut projects {
        if project.name == name || name == "*" {
            let mut dir = PathBuf::from(&base_dir);
            if let Some(category) = &project.category {
                dir.push(category)
            }
            if let Some(p_type) = &project.p_type {
                dir.push(p_type)
            }
            dir.push(project.name.clone());

            let cur_dir = PathBuf::from(project.directory.clone().trim_start());

            if dir != cur_dir {
                if flags.dry_run {
                    println!("{} -- {} --> {}", project.name, cur_dir.display(), dir.display());
                } else if flags.force || flags.interactive {
                    println!("{} -- {} --> {}", project.name, cur_dir.display(), dir.display());
                    if flags.interactive {
                        // ask if the user would like to move this folder
                        let mut input = String::new();
                        println!("Do you wish to move this folder? (y/n)");
                        io::stdin().read_line(&mut input).unwrap();
                        if input.trim() != "y" {
                            continue;
                        }
                    }

                    let mut to_dir = dir.clone();
                    to_dir.pop();
                    move_folder(cur_dir, to_dir).unwrap();

                    project.directory = String::from(dir.to_str().unwrap());

                    println!("Finished moving {}", project.name);
                } else {
                    eprintln!(
                        "Please set one of the process flags to run the refactor. I.E. --dry-run, --force, or \
                         --interactive"
                    );
                    exit(1);
                }
            }
        }
    }
    save_projects(&projects);
}

fn edit_project(mut projects: Vec<Project>, project_name: String, options: EditOptions) {
    let mut filtered_projects: Vec<&mut Project> = projects.iter_mut().filter(|p| p.name == project_name).collect();
    let project_index = if filtered_projects.len() > 1 {
        println!("More than one project found with the name {}", project_name);
        println!("Enter the number for the project you wish to edit");
        for (num, proj) in filtered_projects.iter().enumerate() {
            println!("{}: {} - {}", num, proj.name, proj.directory);
        }
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let num = input.trim().parse::<usize>().unwrap();
        println!(
            "Editing project {} in {}",
            filtered_projects[num].name, filtered_projects[num].directory
        );
        num
    } else {
        0
    };

    if let Some(name) = options.name {
        filtered_projects[project_index].name = name;
    }
    if let Some(directory) = options.directory {
        filtered_projects[project_index].directory = directory;
    }
    if let Some(p_type) = options.p_type {
        filtered_projects[project_index].p_type = Some(p_type);
    }
    if let Some(category) = options.category {
        filtered_projects[project_index].category = Some(category);
    }
    if options.remove_type {
        filtered_projects[project_index].p_type = None;
    }
    if options.remove_category {
        filtered_projects[project_index].category = None;
    }

    save_projects(&projects);
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
