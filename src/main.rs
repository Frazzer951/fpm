use std::fs;
use std::path::PathBuf;
use std::process::{exit, Command as CMD};

use clap::{command, Arg, ArgAction, ArgMatches, Command};
use regex::Regex;

use crate::file_handler::{FileError, Project};
use crate::project_structure::{build_folder, load_template, Folder, TemplateVars};
use crate::settings::Settings;

mod file_handler;
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
        .subcommand(
            Command::new("new")
                .about("Create a New project")
                .arg_required_else_help(true)
                .args(
                    &[
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
                            "Project Category - Another layer of separation, similar to project type, that will help \
                             to get project seperated. Examples would be `Work`, `Personal` and so on",
                        ),
                        Arg::new("directory")
                            .short('d')
                            .long("directory")
                            .takes_value(true)
                            .help(
                                "Manually specify the base directory to use. -- Overrides base_dir specified in config",
                            ),
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
                    ],
                ),
        )
        .subcommand(
            Command::new("add")
                .about("Add an existing project")
                .arg_required_else_help(true)
                .args(
                    &[
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
                    ],
                ),
        )
        .subcommand(
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
                .subcommand(Command::new("init").about("Initialize the config file with default options")),
        )
        .subcommand(
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
                ),
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

            new_project(
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

            add_project(projects, name, directory, p_type, category);
        },
        Some(("config", sub_matches)) => {
            let sub_command = sub_matches.subcommand();

            config_handler(&mut settings, sub_command);
        },
        Some(("project", sub_matches)) => {
            let sub_command = sub_matches.subcommand();

            project_handler(&mut projects, sub_command);
        },
        _ => unreachable!(),
    }
}

fn project_handler(projects: &mut Vec<Project>, command: Option<(&str, &ArgMatches)>) {
    match command {
        Some(("list", sub_matches)) => {
            let filter = sub_matches.get_one::<Regex>("filter").cloned();
            for project in projects {
                if filter.is_none() || filter.as_ref().unwrap().is_match(project.name.as_str()) {
                    println!("{}", project.name);
                }
            }
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

fn add_project(
    mut projects: Vec<Project>,
    name: String,
    directory: String,
    p_type: Option<String>,
    category: Option<String>,
) {
    // is there a folder at directory?
    if !std::path::Path::new(&directory).exists() {
        eprintln!("The directory `{}` specified does not exist", directory);
        exit(1);
    }

    // add project to known projects
    projects.push(Project {
        name,
        directory,
        category,
        p_type,
    });
    file_handler::save_projects(projects);
}

fn new_project(
    mut settings: &mut Settings,
    mut projects: Vec<Project>,
    project_vars: Project,
    git_url: Option<String>,
    templates: Vec<String>,
) {
    let dir = project_vars.directory;
    let mut project_path = std::path::PathBuf::from(dir.clone());
    if project_vars.category.is_some() {
        project_path.push(project_vars.category.as_ref().unwrap());
    }
    if project_vars.p_type.is_some() {
        project_path.push(project_vars.p_type.as_ref().unwrap());
    }
    project_path.push(project_vars.name.clone());

    let mut project = Folder {
        name:        project_vars.name.clone(),
        files:       vec![],
        sub_folders: vec![],
        commands:    vec![],
    };

    if settings.template_dir.is_none() {
        let mut template_path = PathBuf::from(dir);
        template_path.push("templates");
        settings.template_dir = Some(String::from(template_path.to_str().unwrap()));
    }

    for template in templates {
        load_template(settings, &mut project, template.clone());
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
        project_name: project_vars.name.clone(),
    };

    // build the project
    build_folder(project_path.clone(), &project, &template_vars);

    // run git clone command
    if let Some(git_url) = git_url {
        let command = settings.git_command.replace("{FPM_GIT_URL}", git_url.as_str());

        let command_parts: Vec<&str> = command.split(' ').collect();
        // run the command stored in the command variable
        let mut cmd = CMD::new(command_parts[0]);
        cmd.args(&command_parts[1..]);
        cmd.current_dir(project_path.clone());
        cmd.status()
            .unwrap_or_else(|err| panic!("Failed to run the command {} -- {}", command, err));
    }

    // add project to known projects
    projects.push(Project {
        name:      project_vars.name,
        directory: String::from(project_path.to_str().unwrap()),
        category:  project_vars.category.clone(),
        p_type:    project_vars.p_type.clone(),
    });
    file_handler::save_projects(projects);
}
