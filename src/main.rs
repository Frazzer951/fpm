use std::path::PathBuf;
use std::process::exit;

use clap::{command, App, Arg, ArgAction, ArgGroup, ArgMatches, Command};
use path_absolutize::Absolutize;
use regex::Regex;

use crate::file_handler::{save_projects, FileError, Project};
use crate::project::add_project_from_folder;
use crate::project_structure::{build_folder, load_template, Folder, TemplateVars};
use crate::settings::Settings;

mod file_handler;
mod project;
mod project_structure;
mod settings;
mod utils;

// Project Constants
const PROJECT_NAME: &str = "fpm";
const CONFIG_FILENAME: &str = "config.toml";
const PROJECT_DB_FILENAME: &str = "projects_db.json";
const PROJECT_ENV_PREFIX: &str = "FPM";

fn cli() -> Command<'static> {
    command!()
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommands(vec![
            subcommand_new(),
            subcommand_add(),
            subcommand_config(),
            subcommand_project(),
            subcommand_list(),
            subcommand_add_folder(),
        ])
}

fn subcommand_new() -> App<'static> {
    Command::new("new")
        .about("Create a New project")
        .arg_required_else_help(true)
        .args(&[
            Arg::new("name")
                .required(true)
                .short('n')
                .long("name")
                .takes_value(true)
                .help("Project Name"),
            Arg::new("type")
                .short('t')
                .long("type")
                .takes_value(true)
                .help("Project Type - This determines the folder the project will placed into"),
            Arg::new("category").short('c').long("category").takes_value(true).help(
                "Project Category - Another layer of separation, similar to project type, that will help to get \
                 project seperated. Examples would be `Work`, `Personal` and so on",
            ),
            Arg::new("directory")
                .short('d')
                .long("directory")
                .takes_value(true)
                .help("Manually specify the base directory to use. -- Overrides base_dir specified in config"),
            Arg::new("template")
                .long("template")
                .visible_alias("t")
                .takes_value(true)
                .multiple_values(true)
                .action(ArgAction::Append)
                .help("Templates to use when generating a project i.e. `--t template1 template2`"),
            Arg::new("GIT_URL")
                .short('g')
                .long("git-url")
                .takes_value(true)
                .conflicts_with("template")
                .help("Clone from a Git URL"),
            Arg::new("open")
                .short('o')
                .long("open")
                .action(ArgAction::SetTrue)
                .help("Open the project folder after creating it"),
        ])
}

fn subcommand_add() -> App<'static> {
    Command::new("add")
        .about("Add an existing project")
        .arg_required_else_help(true)
        .args(&[
            Arg::new("name")
                .required(true)
                .short('n')
                .long("name")
                .takes_value(true)
                .help("Project Name"),
            Arg::new("directory")
                .required(true)
                .short('d')
                .long("directory")
                .takes_value(true)
                .help("Directory of the project"),
            Arg::new("type")
                .short('t')
                .long("type")
                .takes_value(true)
                .help("Project Type"),
            Arg::new("category")
                .short('c')
                .long("category")
                .takes_value(true)
                .help("Project Category"),
        ])
}

fn subcommand_config() -> App<'static> {
    Command::new("config")
        .about("Configuration Settings")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("set")
                .about("Set the value of a config option")
                .arg_required_else_help(true)
                .args(&[
                    Arg::new("setting")
                        .required(true)
                        .takes_value(true)
                        .value_parser(["base_dir", "template_dir", "git_command"])
                        .help("The setting to modify"),
                    Arg::new("value").required(true).help("The modified value"),
                ]),
        )
        .subcommand(Command::new("init").about("Initialize the config file with default options"))
        .subcommand(Command::new("open").about("Open the config directory"))
}

fn subcommand_project() -> App<'static> {
    Command::new("project")
        .about("Project options")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .arg(
            Arg::new("project_name")
                .takes_value(true)
                .default_value("*")
                .global(true)
                .help("The name of the project to verify or leave blank to verify all projects"),
        )
        .subcommand(
            Command::new("verify")
                .about("Verify that the project in the project database are where the project directory specifies")
                .args(&[
                    Arg::new("list")
                        .short('l')
                        .long("list")
                        .action(ArgAction::SetTrue)
                        .help("List out project that don't pass verification"),
                    Arg::new("remove")
                        .short('r')
                        .long("remove")
                        .action(ArgAction::SetTrue)
                        .help("Remove projects that don't pass verification without warning"),
                    Arg::new("interactive")
                        .short('i')
                        .long("interactive")
                        .action(ArgAction::SetTrue)
                        .help("Interactive mode"),
                ])
                .group(
                    ArgGroup::new("verify_options")
                        .args(&["list", "remove", "interactive"])
                        .required(true),
                ),
        )
        .subcommand(
            Command::new("refactor")
                .about("Move folders to the correct directory based on their name, category, and type")
                .args(&[
                    Arg::new("dry-run")
                        .short('n')
                        .long("dry-run")
                        .action(ArgAction::SetTrue)
                        .help("Do not move any folders yet just see what would change"),
                    Arg::new("force")
                        .short('f')
                        .long("force")
                        .action(ArgAction::SetTrue)
                        .help("Required to move folders when not using Interactive mode"),
                    Arg::new("interactive")
                        .short('i')
                        .long("interactive")
                        .action(ArgAction::SetTrue)
                        .help("Ask the user for each folder if it should move"),
                    Arg::new("directory")
                        .short('d')
                        .long("directory")
                        .takes_value(true)
                        .help("Manually specify the base directory to use. -- Overrides base_dir specified in config"),
                    Arg::new("verbose")
                        .short('v')
                        .long("verbose")
                        .action(ArgAction::SetTrue)
                        .help("Print out what files are being moved"),
                ]),
        )
        .subcommand(
            Command::new("edit").about("edit a project").args(&[
                Arg::new("name")
                    .short('n')
                    .long("name")
                    .takes_value(true)
                    .help("Change the project's name"),
                Arg::new("directory")
                    .short('d')
                    .long("directory")
                    .takes_value(true)
                    .help("Change the project's directory. DOES NOT MOVE THE PROJECT"),
                Arg::new("type")
                    .short('t')
                    .long("type")
                    .takes_value(true)
                    .help("Change the project's type"),
                Arg::new("category")
                    .short('c')
                    .long("category")
                    .takes_value(true)
                    .help("Change the project's category"),
                Arg::new("remove_type")
                    .long("remove-type")
                    .conflicts_with("type")
                    .action(ArgAction::SetTrue)
                    .help("Remove the project's type"),
                Arg::new("remove_category")
                    .long("remove-category")
                    .conflicts_with("category")
                    .action(ArgAction::SetTrue)
                    .help("Remove the project's category"),
            ]),
        )
}

fn subcommand_list() -> App<'static> {
    Command::new("list").about("List out all known projects").arg(
        Arg::new("filter")
            .short('f')
            .long("filter")
            .takes_value(true)
            .value_parser(clap::value_parser!(Regex))
            .help("A regex filter used to filter names when displaying projects"),
    )
}

fn subcommand_add_folder() -> App<'static> {
    Command::new("add-folder")
        .about("Interactively add folders from the specified directory")
        .args(&[
            Arg::new("path")
                .takes_value(true)
                .required(true)
                .value_parser(clap::value_parser!(PathBuf))
                .help("The Path to the directory to add folders from"),
            Arg::new("type")
                .short('t')
                .long("type")
                .takes_value(true)
                .help("Project Type"),
            Arg::new("category")
                .short('c')
                .long("category")
                .takes_value(true)
                .help("Project Category"),
        ])
}

fn main() {
    // Parse the command line arguments
    let matches = cli().get_matches();

    // Load the config file
    let mut settings = Settings::new();

    // Load the project database
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
            for project in projects {
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
