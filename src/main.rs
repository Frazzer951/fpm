use std::str::FromStr;
use std::{fmt, fs};

use clap::{ArgEnum, Parser, Subcommand};
use serde::{Deserialize, Serialize};
use serde_json;

// region -- Project Constants
const PROJECT_NAME: &str = "fpm";
const CONFIG_FILENAME: &str = "config.toml";
const PROJECT_DB_FILENAME: &str = "projects_db.json";
// endregion

// region -- Custom Errors
type Result<T> = std::result::Result<T, FileError>;

#[derive(Debug, Clone)]
enum FileError {
    LoadingError,
    ParsingError,
}

impl fmt::Display for FileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FileError::LoadingError => {
                write!(f, "Failed to load config file")
            },
            FileError::ParsingError => {
                write!(f, "Failed to parse config file")
            },
        }
    }
}
// endregion

// region -- Config Struct
#[derive(Deserialize, Serialize, Default, Debug)]
struct Config {
    #[serde(default)]
    base_dir: Option<String>,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ArgEnum)]
enum ConfigOptions {
    BaseDir,
}
// endregion

// region -- Project Struct
#[derive(Deserialize, Serialize, Default, Debug)]
struct Project {
    name:      String,
    directory: String,
    #[serde(default)]
    category:  Option<String>,
    #[serde(default)]
    p_type:    Option<String>,
}
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

    let mut config = match load_config() {
        Ok(c) => c,
        Err(FileError::LoadingError) => {
            eprintln!("Failed to load the config file using default settings");
            Config::default()
        },
        Err(FileError::ParsingError) => {
            eprintln!(
                "The Config file failed to parse, please check for any errors in the file and re-run your command."
            );
            return;
        },
    };

    let mut projects = match load_projects() {
        Ok(p) => p,
        Err(FileError::LoadingError) => {
            vec![]
        },
        Err(FileError::ParsingError) => {
            eprintln!(
                "The Projects Database file failed to parse, please check for any errors in the file and re-run your \
                 command."
            );
            return;
        },
    };

    match &cli.command {
        Commands::New {
            name,
            p_type,
            category,
            directory,
        } => {
            if config.base_dir.is_none() && directory.is_none() {
                eprintln!("No directory was specified, and the global Base Directory is not Set.");
                eprintln!("Specify a directory in the command, or Set a global directory with the config command`");
                return;
            }
            let dir = directory.as_ref().unwrap_or_else(|| config.base_dir.as_ref().unwrap());
            let mut project_path = std::path::PathBuf::from_str(dir).unwrap();
            if p_type.is_some() {
                project_path.push(p_type.as_ref().unwrap());
            }
            if category.is_some() {
                project_path.push(category.as_ref().unwrap());
            }
            project_path.push(name);

            // create project folders
            fs::create_dir_all(project_path.clone()).unwrap();

            // add project to known projects
            projects.push(Project {
                name:      name.clone(),
                directory: String::from(project_path.to_str().unwrap()),
                category:  category.clone(),
                p_type:    p_type.clone(),
            });
            save_projects(projects);
        },
        Commands::Config { command } => match &command {
            ConfigCommands::Set { setting, value } => {
                match &setting {
                    ConfigOptions::BaseDir => {
                        config.base_dir = Some(value.clone());
                    },
                }

                save_config(config);
            },
            ConfigCommands::Init => save_config(config),
        },
    }
}

fn load_config() -> Result<Config> {
    let mut config_dir = dirs::config_dir().unwrap();
    config_dir.push(PROJECT_NAME);
    config_dir.push(CONFIG_FILENAME);

    let contents = match fs::read_to_string(config_dir) {
        Ok(c) => Ok(c),
        Err(_) => Err(FileError::LoadingError),
    }?;

    match toml::from_str(&contents) {
        Ok(d) => Ok(d),
        Err(_) => Err(FileError::ParsingError),
    }
}

fn load_projects() -> Result<Vec<Project>> {
    let mut projects_dir = dirs::config_dir().unwrap();
    projects_dir.push(PROJECT_NAME);
    projects_dir.push(PROJECT_DB_FILENAME);

    let contents = match fs::read_to_string(projects_dir) {
        Ok(c) => Ok(c),
        Err(_) => Err(FileError::LoadingError),
    }?;

    match serde_json::from_str(&contents) {
        Ok(d) => Ok(d),
        Err(_) => Err(FileError::ParsingError),
    }
}

fn save_config(config: Config) {
    let mut config_dir = dirs::config_dir().unwrap();
    config_dir.push(PROJECT_NAME);

    // make sure path exists
    fs::create_dir_all(config_dir.clone()).unwrap();

    config_dir.push(CONFIG_FILENAME);

    // save config to config_dir
    let contents = toml::to_string(&config).unwrap();
    fs::write(config_dir, contents).unwrap();
}

fn save_projects(projects: Vec<Project>) {
    let mut projects_dir = dirs::config_dir().unwrap();
    projects_dir.push(PROJECT_NAME);

    // make sure path exists
    fs::create_dir_all(projects_dir.clone()).unwrap();

    projects_dir.push(PROJECT_DB_FILENAME);

    // save config to config_dir
    let contents = serde_json::to_string(&projects).unwrap();
    fs::write(projects_dir, contents).unwrap();
}
