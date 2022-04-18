use clap::{Parser, Subcommand};
use serde_json;

#[derive(Parser)]
#[clap(version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new project directory
    Create {
        name: String,
        #[clap(short, long)]
        language: String,
        #[clap(short, long)]
        template: bool,
        #[clap(short, long)]
        git_repo: bool,
        #[clap(short, long)]
        open: bool,
    },
}

fn load_template(language: &String) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let filename = format!("templates/{language}.json");
    let f = std::fs::File::open(filename)?;
    let d: serde_json::Value = serde_json::from_reader(f)?;
    Ok(d)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Create {
            name,
            language,
            template,
            ..
        } => {
            println!("Creating project named {}", name);
            if *template {
                let yaml_template = load_template(language)?;
                println!("{:?}", yaml_template["c++"]);
            }
        }
    }

    Ok(())
}
