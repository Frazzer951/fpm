use clap::{Parser, Subcommand};
use serde::Deserialize;
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

fn load_template(language: &String) -> Result<Template, Box<dyn std::error::Error>> {
    let filename = format!("templates/{language}.json");
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

            let mut proj_folder = Folder {
                name: name.clone(),
                files: vec![File {
                    name: "README.md".to_string(),
                    lines: vec![format!("# {name}")],
                }],
                sub_folders: None,
            };

            if *template {
                let yaml_template: Template = load_template(language)?;
                //println!("{:#?}", yaml_template);

                for file in yaml_template.files.unwrap() {
                    //println!("file: {:#?}", file);
                    proj_folder.files.push(file);
                }

                for folder in yaml_template.folders.unwrap() {
                    //println!("folder: {:#?}", folder);
                    proj_folder.add_sub_folder(folder);
                }
            }

            //println!("{:#?}", proj_folder);
        }
    }

    Ok(())
}
