use std::path::PathBuf;

use clap::{command, App, Arg, ArgAction, ArgGroup, Command};
use regex::Regex;

pub fn cli() -> Command<'static> {
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
                Arg::new("refactor")
                    .long("refactor")
                    .short('r')
                    .action(ArgAction::SetTrue)
                    .help("Refactor the project after editing it"),
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
