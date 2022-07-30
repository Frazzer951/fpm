#![feature(no_coverage)]

use std::path::PathBuf;
use std::process::exit;

use clap::ArgMatches;
use path_absolutize::Absolutize;
use regex::Regex;

use crate::file_handler::{FileError, Project, Projects};
use crate::project::add_project_from_folder;
use crate::settings::Settings;

pub mod cli;
pub mod file_handler;
pub mod project;
pub mod settings;
pub mod templates;
pub mod utils;

// Project Constants
pub const PROJECT_NAME: &str = "fpm";
pub const CONFIG_FILENAME: &str = "config.toml";
pub const PROJECT_DB_FILENAME: &str = "projects_db.json";
pub const PROJECT_ENV_PREFIX: &str = "FPM";

pub fn parse() {
    // Parse the command line arguments
    let matches = cli::cli().get_matches();

    // Load the config file
    let mut settings = Settings::new();

    // Load the project database
    let mut projects = match Projects::load(Projects::default_dir()) {
        Ok(p) => p,
        Err(FileError::LoadingError) => Projects::default(),
        Err(FileError::ParsingError) => {
            eprintln!(
                "The Projects Database file failed to parse, please check for any errors in the file and re-run your \
                 command."
            );
            exit(1);
        },
    };

    // Match the subcommands
    match matches.subcommand() {
        Some(("new", sub_matches)) => {
            // Load all the variables
            let name = sub_matches.get_one::<String>("name").expect("REQUIRED").clone();
            let p_type = sub_matches.get_one::<String>("type").cloned();
            let category = sub_matches.get_one::<String>("category").cloned();
            let directory = sub_matches.get_one::<String>("directory").cloned();
            let git_url = sub_matches.get_one::<String>("GIT_URL").cloned();
            let open = sub_matches.get_one::<bool>("open").cloned().expect("BOOL VALUE");
            let templates = sub_matches
                .get_many::<String>("template")
                .into_iter()
                .flatten()
                .cloned()
                .collect::<Vec<_>>();

            // Error if no directory is specified
            if settings.base_dir.is_none() && directory.is_none() {
                eprintln!("No directory was specified, and the global Base Directory is not Set.");
                eprintln!("Specify a directory in the command, or Set a global directory with the config command`");
                return;
            }

            // Set the directory to the global base directory if it was not specified
            let dir = directory.unwrap_or_else(|| settings.base_dir.as_ref().unwrap().clone());

            // Create the project
            project::new_project(
                &mut settings,
                projects,
                Project {
                    name,
                    directory: dir,
                    category,
                    p_type,
                },
                git_url,
                templates,
                open,
            );
        },
        Some(("add", sub_matches)) => {
            // Load all the variables
            let name = sub_matches.get_one::<String>("name").expect("REQUIRED").clone();
            let directory = sub_matches.get_one::<String>("directory").expect("REQUIRED").clone();
            let p_type = sub_matches.get_one::<String>("type").cloned();
            let category = sub_matches.get_one::<String>("category").cloned();

            // Add the project to the database
            project::add_project(&mut projects, name, directory, p_type, category);
        },
        Some(("config", sub_matches)) => {
            // Load the subcommands and pass it to the config handler
            let sub_command = sub_matches.subcommand();

            config_handler(&mut settings, sub_command);
        },
        Some(("project", sub_matches)) => {
            // Load the subcommands and pass it to the project handler
            let sub_command = sub_matches.subcommand();
            let project_name = sub_matches
                .get_one::<String>("project_name")
                .expect("Has Default Value")
                .clone();

            // Make sure the project exists and offer possible fixes if it doesn't
            let project_names = project::get_similar(&projects, &project_name);
            if project_name != "*" && project_names.0 != 0 {
                println!("No project with the name {} was found", project_name);
                println!("Did you mean on of the following: {:?}", project_names.1);
                exit(1);
            }

            project::project_handler(&mut projects, project_name, settings, sub_command);
        },
        Some(("list", sub_matches)) => {
            // Load the filter and print the projects
            let filter = sub_matches.get_one::<Regex>("filter").cloned();
            for project in projects.projects {
                if filter.is_none() || filter.as_ref().unwrap().is_match(project.name.as_str()) {
                    println!("{}", project.name);
                }
            }
        },
        Some(("add-folder", sub_matches)) => {
            // Load the variables
            let input_path = sub_matches.get_one::<PathBuf>("path").expect("REQUIRED").clone();
            let path = input_path.absolutize().unwrap().to_path_buf();
            let p_type = sub_matches.get_one::<String>("type").cloned();
            let category = sub_matches.get_one::<String>("category").cloned();

            add_project_from_folder(projects, path, p_type, category);
        },
        _ => unreachable!(),
    }
}

fn config_handler(settings: &mut Settings, command: Option<(&str, &ArgMatches)>) {
    match command {
        Some(("set", sub_matches)) => {
            // Load the setting and value
            let setting = sub_matches.get_one::<String>("setting").expect("REQUIRED").clone();
            let value = sub_matches.get_one::<String>("value").expect("REQUIRED").clone();

            // Set the specified setting
            match setting.as_str() {
                "base_dir" => {
                    settings.base_dir = Some(value);
                },
                "template_dir" => {
                    settings.template_dir = Some(value);
                },
                "git_command" => {
                    settings.git_command = value;
                },
                _ => unreachable!(),
            }

            settings.save();
        },
        Some(("init", _sub_matches)) => settings.save(),
        Some(("open", _sub_matches)) => {
            let mut config_dir = PathBuf::from(settings.config_dir.clone());
            config_dir.pop();

            opener::open(config_dir).expect("Failed to open the directory")
        },
        _ => unreachable!(),
    }
}
