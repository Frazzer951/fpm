use std::fs;
use std::process::exit;
use std::str::FromStr;

use clap::{Parser, Subcommand};
use file_handler::{Config, ConfigOptions, FileError, Project};

use crate::project_structure::{build_folder, load_template, Folder, TemplateVars};

mod file_handler;
mod project_structure;

// region -- Project Constants
const PROJECT_NAME: &str = "fpm";
const CONFIG_FILENAME: &str = "config.toml";
const PROJECT_DB_FILENAME: &str = "projects_db.json";
// endregion

// region -- CLI Structs
#[derive(Parser)]
#[clap(version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a New project
    New {
        #[clap(short, long)]
        /// Project Name
        name:      String,
        #[clap(short = 't', long = "type", value_name = "TYPE")]
        /// Project Type - This determines the folder the project will placed into
        p_type:    Option<String>,
        #[clap(short, long)]
        /// Project Category - Another layer of separation, similar to project type, that will help to get project
        /// seperated. Examples would be `Work`, `Personal` and so on
        category:  Option<String>,
        #[clap(short, long)]
        /// Manually specify the base directory to use. -- Overrides base_dir specified in config
        directory: Option<String>,
        #[clap(long, visible_alias = "t")]
        /// A Template to use when generating a project
        templates: Vec<String>,
    },
    /// Add an existing project
    Add {
        #[clap(short, long)]
        /// Project Name
        name:      String,
        /// The Directory of the project
        #[clap(short, long)]
        directory: String,
        #[clap(short = 't', long = "type", value_name = "TYPE")]
        /// Project Type - This determines the folder the project will placed into
        p_type:    Option<String>,
        #[clap(short, long)]
        /// Project Category - Another layer of separation, similar to project type, that will help to get project
        /// seperated. Examples would be `Work`, `Personal` and so on
        category:  Option<String>,
    },
    /// Configuration Settings
    Config {
        #[clap(subcommand)]
        command: ConfigCommands,
    },
}

#[derive(Subcommand)]
enum ConfigCommands {
    /// Set the value of a config option
    Set {
        #[clap(arg_enum)]
        /// The setting to modify
        setting: ConfigOptions,
        /// The modified value
        value:   String,
    },
    /// Initialize the config file with default options
    Init,
}
// endregion

fn main() {
    let cli = Cli::parse();

    let mut config = match file_handler::load_config() {
        Ok(c) => c,
        Err(FileError::LoadingError) => {
            eprintln!("Failed to load the config file using default settings");
            Config::default()
        },
        Err(FileError::ParsingError) => {
            eprintln!(
                "The Config file failed to parse, please check for any errors in the file and re-run your command."
            );
            exit(1);
        },
    };

    let mut projects = match file_handler::load_projects() {
        Ok(p) => p,
        Err(FileError::LoadingError) => {
            vec![]
        },
        Err(FileError::ParsingError) => {
            eprintln!(
                "The Projects Database file failed to parse, please check for any errors in the file and re-run your \
                 command."
            );
            exit(1);
        },
    };

    match &cli.command {
        Commands::New {
            name,
            p_type,
            category,
            directory,
            templates,
        } => {
            if config.base_dir.is_none() && directory.is_none() {
                eprintln!("No directory was specified, and the global Base Directory is not Set.");
                eprintln!("Specify a directory in the command, or Set a global directory with the config command`");
                return;
            }
            let dir = directory.as_ref().unwrap_or_else(|| config.base_dir.as_ref().unwrap());
            let mut project_path = std::path::PathBuf::from_str(dir).unwrap();
            if category.is_some() {
                project_path.push(category.as_ref().unwrap());
            }
            if p_type.is_some() {
                project_path.push(p_type.as_ref().unwrap());
            }
            project_path.push(name);

            let mut project = Folder {
                name:        name.clone(),
                files:       vec![],
                sub_folders: vec![],
                commands:    vec![],
            };

            for template in templates {
                load_template(&mut project, template.clone());
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
                project_name: name.clone(),
            };

            // build the project
            build_folder(project_path.clone(), &project, &template_vars);

            // add project to known projects
            projects.push(Project {
                name:      name.clone(),
                directory: String::from(project_path.to_str().unwrap()),
                category:  category.clone(),
                p_type:    p_type.clone(),
            });
            file_handler::save_projects(projects);
        },
        Commands::Add {
            name,
            directory,
            p_type,
            category,
        } => {
            // is there a folder at directory?
            if !std::path::Path::new(directory).exists() {
                eprintln!("The directory `{}` specified does not exist", directory);
                exit(1);
            }

            // add project to known projects
            projects.push(Project {
                name:      name.clone(),
                directory: directory.clone(),
                category:  category.clone(),
                p_type:    p_type.clone(),
            });
            file_handler::save_projects(projects);
        },
        Commands::Config { command } => match &command {
            ConfigCommands::Set { setting, value } => {
                match &setting {
                    ConfigOptions::BaseDir => {
                        config.base_dir = Some(value.clone());
                    },
                }

                file_handler::save_config(config);
            },
            ConfigCommands::Init => file_handler::save_config(config),
        },
    }
}
