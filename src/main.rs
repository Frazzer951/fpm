use std::fmt;
use std::fs;

use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use toml;

// region -- Project Constants
const PROJECT_NAME: &str = "fpm";
// endregion

// region -- Custom Errors
type Result<T> = std::result::Result<T, ConfigError>;

#[derive(Debug, Clone)]
enum ConfigError { LoadingError, ParsingError }

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ConfigError::LoadingError => { write!(f, "Failed to load config file") }
            ConfigError::ParsingError => { write!(f, "Failed to parse config file") }
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
        name: String,
        #[clap(short = 't', long = "type", value_name = "TYPE")]
        p_type: Option<String>,
        #[clap(short, long)]
        category: Option<String>,
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
        }
        Err(_) => todo!("Config Error Handling"),
    };

    println!("{:#?}", config);

    match &cli.command {
        Commands::New { name, p_type, category, directory } => {}
    }
}

fn load_config() -> Result<Config> {
    let mut config_dir = dirs::config_dir().unwrap();
    config_dir.push(PROJECT_NAME);
    config_dir.push("config.toml");
    println!("{:?}", config_dir);

    let contents = match fs::read_to_string(config_dir) {
        Ok(c) => Ok(c),
        Err(_) => {
            Err(ConfigError::LoadingError)
        }
    }?;

    match toml::from_str(&contents) {
        Ok(d) => Ok(d),
        Err(_) => {
            Err(ConfigError::ParsingError)
        }
    }
}