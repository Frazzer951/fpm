use std::process::exit;

use clap::{command, App, Arg, ArgAction, ArgMatches, Command};
use regex::Regex;

use crate::file_handler::{save_projects, FileError, Project};
use crate::project_structure::{build_folder, load_template, Folder, TemplateVars};
use crate::settings::Settings;

mod file_handler;
mod project;
mod project_structure;
mod settings;

// region -- Project Constants
const PROJECT_NAME: &str = "fpm";
const CONFIG_FILENAME: &str = "config.toml";
const PROJECT_DB_FILENAME: &str = "projects_db.json";
const PROJECT_ENV_PREFIX: &str = "FPM";
// endregion

fn cli() -> Command<'static> {
    command!()
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommands(vec![
            subcommand_new(),
            subcommand_add(),
            subcommand_config(),
            subcommand_project(),
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
                .args(
                    &[
                        Arg::new("setting")
                            .required(true)
                            .takes_value(true)
                            .value_parser(["base_dir", "template_dir"])
                            .help("The setting to modify"),
                        Arg::new("value").required(true).help("The modified value"),
                    ],
                ),
        )
        .subcommand(Command::new("init").about("Initialize the config file with default options"))
}

fn subcommand_project() -> App<'static> {
    Command::new("project")
        .about("Project options")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("list").about("List out all known projects").arg(
                Arg::new("filter")
                    .short('f')
                    .long("filter")
                    .takes_value(true)
                    .value_parser(clap::value_parser!(Regex))
                    .help("A regex filter used to filter names when displaying projects"),
            ),
        )
        .subcommand(
            Command::new("verify")
                .about("Verify that the project in the project database are where the project directory specifies")
                .args(&[Arg::new("project_name")
                    .takes_value(true)
                    .default_value("*")
                    .help("The name of the project to verify or leave blank to verify all projects")]),
        )
}

fn main() {
    let matches = cli().get_matches();

    let mut settings = Settings::new();

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

    match matches.subcommand() {
        Some(("new", sub_matches)) => {
            let name = sub_matches.get_one::<String>("name").expect("REQUIRED").clone();
            let p_type = sub_matches.get_one::<String>("type").cloned();
            let category = sub_matches.get_one::<String>("category").cloned();
            let directory = sub_matches.get_one::<String>("directory").cloned();
            let git_url = sub_matches.get_one::<String>("GIT_URL").cloned();
            let templates = sub_matches
                .get_many::<String>("template")
                .into_iter()
                .flatten()
                .cloned()
                .collect::<Vec<_>>();

            if settings.base_dir.is_none() && directory.is_none() {
                eprintln!("No directory was specified, and the global Base Directory is not Set.");
                eprintln!("Specify a directory in the command, or Set a global directory with the config command`");
                return;
            }

            let dir = directory.unwrap_or_else(|| settings.base_dir.as_ref().unwrap().clone());

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
            );
        },
        Some(("add", sub_matches)) => {
            let name = sub_matches.get_one::<String>("name").expect("REQUIRED").clone();
            let directory = sub_matches.get_one::<String>("directory").expect("REQUIRED").clone();
            let p_type = sub_matches.get_one::<String>("type").cloned();
            let category = sub_matches.get_one::<String>("category").cloned();

            project::add_project(projects, name, directory, p_type, category);
        },
        Some(("config", sub_matches)) => {
            let sub_command = sub_matches.subcommand();

            config_handler(&mut settings, sub_command);
        },
        Some(("project", sub_matches)) => {
            let sub_command = sub_matches.subcommand();

            project::project_handler(&mut projects, sub_command);
        },
        _ => unreachable!(),
    }
}

fn config_handler(settings: &mut Settings, command: Option<(&str, &ArgMatches)>) {
    match command {
        Some(("set", sub_matches)) => {
            let setting = sub_matches.get_one::<String>("setting").expect("REQUIRED").clone();
            let value = sub_matches.get_one::<String>("value").expect("REQUIRED").clone();

            match setting.as_str() {
                "base_dir" => {
                    settings.base_dir = Some(value);
                },
                "template_dir" => {
                    settings.template_dir = Some(value);
                },
                _ => unreachable!(),
            }

            settings.save();
        },
        Some(("init", _sub_matches)) => settings.save(),
        _ => unreachable!(),
    }
}
