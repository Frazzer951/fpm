use std::io::Write;

use clap::{Parser, Subcommand};
use serde::Deserialize;

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

fn load_template(template_name: &str) -> Result<Template, Box<dyn std::error::Error>> {
    let filename = format!("templates/{template_name}.json");
    let f = std::fs::File::open(filename)?;
    let d: Template = serde_json::from_reader(f)?;
    Ok(d)
}

#[derive(Debug, Deserialize)]
struct Template {
    folders: Option<Vec<Folder>>,
    files: Option<Vec<File>>,
}

#[derive(Debug, Deserialize)]
struct Folder {
    name: String,
    files: Vec<File>,
    sub_folders: Option<Vec<Folder>>,
}

#[derive(Debug, Deserialize)]
struct File {
    name: String,
    lines: Vec<String>,
}

impl Folder {
    pub fn add_sub_folder(&mut self, folder: Folder) {
        self.sub_folders.get_or_insert_with(Vec::new).push(folder);
    }
}

fn process_folder(path: std::path::PathBuf, folder: &Folder, proj_name: &str) -> std::io::Result<()> {
    if folder.sub_folders.is_some() {
        for f in folder.sub_folders.as_ref().unwrap() {
            let mut new_path = path.clone();
            new_path.push(f.name.clone());
            std::fs::create_dir_all(&new_path)?;
            process_folder(new_path, f, proj_name)?;
        }
    }
    for f in &folder.files {
        create_file(path.clone(), f, proj_name)?;
    }
    Ok(())
}

fn create_file(mut path: std::path::PathBuf, file: &File, proj_name: &str) -> std::io::Result<()> {
    path.push(file.name.clone());
    //println!("{:#?}", path);
    let mut f = std::fs::File::create(path)?;
    for line in &file.lines {
        let line = line.replace("<name>", proj_name) + "\n";
        f.write_all(line.as_bytes())?;
    }

    Ok(())
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
                println!("{}", template_name);
                let template_name = if template_name.is_empty() { language } else { template_name };
                println!("{}", template_name);

                let yaml_template: Template = load_template(template_name)?;
                //println!("{:#?}", yaml_template);

                if yaml_template.files.is_some() {
                    for file in yaml_template.files.unwrap() {
                        //println!("file: {:#?}", file);
                        proj_folder.files.push(file);
                    }
                }

                if yaml_template.folders.is_some() {
                    for folder in yaml_template.folders.unwrap() {
                        //println!("folder: {:#?}", folder);
                        proj_folder.add_sub_folder(folder);
                    }
                }
            }

            //println!("{:#?}", proj_folder);

            let mut base = base_dir.clone();
            base.push(language);
            base.push(name);
            //println!("{:#?}", base);

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
