use clap::{Parser, Subcommand};

mod folder;
mod template;

use folder::process_folder;
use template::{load_template, File, Folder, Template};

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
        /// The name of the new project
        name: String,
        #[clap(short, long)]
        /// The language that the project will use.
        language: String,
        #[clap(short, long, parse(from_os_str), default_value = ".")]
        /// The base directory to place the project folders into
        base_dir: std::path::PathBuf,
        #[clap(short, long)]
        /// Whether to use the template for the language
        template: bool,
        #[clap(short('n'), long, default_value = "")]
        /// Specify specific template, leave blank for default template
        template_name: String,
        #[clap(short, long)]
        /// Create a git repo for the project
        git_repo: bool,
        #[clap(short, long)]
        /// Open the folder when done
        open: bool,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Create {
            name,
            language,
            base_dir,
            template,
            template_name,
            git_repo,
            open,
        } => {
            println!("Creating project named {}", name);

            let mut proj_folder = Folder {
                name: name.clone(),
                files: vec![File {
                    name: "README.md".to_string(),
                    lines: vec![format!("# {name}")],
                }],
                sub_folders: None,
            };

            if *template {
                let template_name = if template_name.is_empty() { language } else { template_name };

                let yaml_template: Template = load_template(template_name)?;

                if yaml_template.files.is_some() {
                    for file in yaml_template.files.unwrap() {
                        proj_folder.files.push(file);
                    }
                }

                if yaml_template.folders.is_some() {
                    for folder in yaml_template.folders.unwrap() {
                        proj_folder.add_sub_folder(folder);
                    }
                }
            }

            let mut base = base_dir.clone();
            base.push(language);
            base.push(name);

            std::fs::create_dir_all(&base).unwrap();

            process_folder(base.clone(), &proj_folder, name)?;

            // open project folder
            if *open {
                std::process::Command::new("explorer").arg(base.to_str().unwrap()).spawn()?;
            }

            // initialize git
            if *git_repo {
                std::process::Command::new("git").arg("init").current_dir(&base).spawn()?;
            }
        }
    }

    Ok(())
}
