use crate::interactive_inputs;
use crate::utils::{create_spinner, Error, Result};
use clap::{command, value_parser, Arg, ArgAction, Command};
use lib_fpm::{config::Config, project::Project};
use std::path::PathBuf;

fn cli() -> Command {
    command!()
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommands(vec![subcommand_new()])
}

fn subcommand_new() -> Command {
    Command::new("new").about("Create a New Project").args(&[
        Arg::new("name").short('n').long("name").help("Project Name"),
        Arg::new("desc").long("desc").help("Description of the project"),
        Arg::new("directory")
            .short('d')
            .long("directory")
            .help("The directory to place the project in. If nothing is provided a directory will be generated")
            .value_parser(value_parser!(PathBuf)),
        Arg::new("tags")
            .long("tag")
            .num_args(1..)
            .action(ArgAction::Append)
            .help("Tags for the project"),
        Arg::new("language")
            .short('l')
            .long("language")
            .help("Primary programming language used"),
        Arg::new("category")
            .short('c')
            .long("category")
            .help("Used to keep similar project types together. I.E. `work`, `thirdparty`, etc"),
        Arg::new("interactive")
            .short('i')
            .long("interactive")
            .action(ArgAction::SetTrue),
    ])
}

pub fn parse() -> Result<()> {
    let matches = cli().get_matches();

    let config = Config::load()?;

    match matches.subcommand() {
        Some(("new", sub_matches)) => {
            let dir = sub_matches.get_one::<PathBuf>("directory").cloned();

            let mut name = sub_matches.get_one::<String>("name").cloned();
            let mut desc = sub_matches.get_one::<String>("desc").cloned();
            let mut language = sub_matches.get_one::<String>("language").cloned();
            let mut category = sub_matches.get_one::<String>("category").cloned();
            let mut tags = sub_matches
                .get_many::<String>("tags")
                .into_iter()
                .flatten()
                .cloned()
                .collect::<Vec<_>>();
            let interactive = sub_matches.get_flag("interactive");

            if interactive {
                let new_params = interactive_inputs::new_params_interactive(name, desc, tags, language, category)?;

                name = new_params.name;
                desc = new_params.desc;
                tags = new_params.tags;
                language = new_params.language;
                category = new_params.category;

                println!("\n\n");
                println!("Name: {name:?}");
                println!("Desc: {desc:?}");
                println!("Tags: {tags:?}");
                println!("Language: {language:?}");
                println!("Category: {category:?}");
            }

            if name.is_none() {
                println!("A name is required for a project, please specify one");
                return Ok(());
            }

            let mut project = Project::new(name, desc, tags, language, category);

            let pb = create_spinner("Creating Folder...")?;
            match project.build(dir, config) {
                Ok(_) => {},
                Err(e) => match e {
                    lib_fpm::error::Error::ConfigMissingValue(e) => {
                        println!("Missing a value for `{e}`, either set it in the config, or pass a directory through the command line");
                        return Ok(());
                    },
                    e => return Err(Error::Fpm(e)),
                },
            };
            pb.finish_with_message("Folder Created");

            // TODO: Save Project to database

            println!("{project:#?}");
        },
        _ => unreachable!(),
    }

    Ok(())
}
