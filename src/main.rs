use std::str::FromStr;
use std::{fmt, fs};

use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};

// region -- Project Constants
const PROJECT_NAME: &str = "fpm";
// endregion

// region -- Custom Errors
type Result<T> = std::result::Result<T, ConfigError>;

#[derive(Debug, Clone)]
enum ConfigError {
    LoadingError,
    ParsingError,
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ConfigError::LoadingError => {
                write!(f, "Failed to load config file")
            },
            ConfigError::ParsingError => {
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
        name:      String,
        #[clap(short = 't', long = "type", value_name = "TYPE")]
        p_type:    Option<String>,
        #[clap(short, long)]
        category:  Option<String>,
        #[clap(short, long)]
        directory: Option<String>,
    },
}
// endregion

fn main() {
    let cli = Cli::parse();

    let config = match load_config() {
        Ok(c) => c,
        Err(ConfigError::LoadingError) => {
            eprintln!("Failed to load the config file using default settings");
            Config::default()
        },
        Err(_) => todo!("Config Error Handling"),
    };

    match &cli.command {
        Commands::New {
            name,
            p_type,
            category,
            directory,
        } => {
            if config.base_dir.is_none() && directory.is_none() {
                eprintln!("No directory was specified, and the global Base Directory is not set.");
                eprintln!("Specify a directory in the command, or set a global directory with the config command`");
                return;
            }
            let dir = directory.as_ref().unwrap_or_else(|| config.base_dir.as_ref().unwrap());
            let mut project_path = std::path::PathBuf::from_str(dir).unwrap();
            if p_type.is_some() {
                project_path.push(p_type.as_ref().unwrap());
            }
            project_path.push(name);

            // create project folders
            fs::create_dir_all(project_path.clone()).unwrap();
        },
    }
}

fn load_config() -> Result<Config> {
    let mut config_dir = dirs::config_dir().unwrap();
    config_dir.push(PROJECT_NAME);
    config_dir.push("config.toml");

    let contents = match fs::read_to_string(config_dir) {
        Ok(c) => Ok(c),
        Err(_) => Err(ConfigError::LoadingError),
    }?;

    match toml::from_str(&contents) {
        Ok(d) => Ok(d),
        Err(_) => Err(ConfigError::ParsingError),
    }
}
