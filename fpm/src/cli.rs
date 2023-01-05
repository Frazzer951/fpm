use crate::utils::{Error, Result};
use clap::{command, value_parser, Arg, ArgAction, Command};
use console::Term;
use dialoguer::{theme::ColorfulTheme, Input};
use indicatif::{ProgressBar, ProgressStyle};
use lib_fpm::{config::Config, project::Project};
use std::path::PathBuf;
use std::time::Duration;

use std::thread;

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
                let new_params = new_params_interactive(name, desc, tags, language, category)?;

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

            let pb = ProgressBar::new_spinner();
            pb.enable_steady_tick(Duration::from_millis(80));
            pb.set_style(
                ProgressStyle::with_template("{spinner:.blue} {msg}")?.tick_strings(&["⣾", "⣽", "⣻", "⢿", "⡿", "⣟", "⣯", "⣷"]),
            );

            pb.set_message("Creating Folder...");

            // TODO: Create Project folder
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

            thread::sleep(Duration::from_secs(5));

            pb.finish_with_message("Folder Created");

            // TODO: Save Project to database

            println!("{project:#?}");
        },
        _ => unreachable!(),
    }

    Ok(())
}

struct NewParams {
    name: Option<String>,
    desc: Option<String>,
    tags: Vec<String>,
    language: Option<String>,
    category: Option<String>,
}

fn new_params_interactive(
    name: Option<String>,
    desc: Option<String>,
    mut tags: Vec<String>,
    language: Option<String>,
    category: Option<String>,
) -> Result<NewParams> {
    // Get Name
    let name = Some(
        Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Project Name")
            .with_initial_text(name.unwrap_or_default())
            .interact_text()?,
    );

    // Get Description
    let mut desc: Option<String> = Some(
        Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Project Desc")
            .with_initial_text(desc.unwrap_or_default())
            .allow_empty(true)
            .interact_text()?,
    );

    // Set to None if Empty
    if let Some(d) = &desc {
        if d.is_empty() {
            desc = None;
        }
    };

    // Get Tags
    let term = Term::stdout();
    loop {
        term.write_line(&format!("Current tags are: {tags:?}"))?;
        let tag: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Tags (leave empty to continue)")
            .allow_empty(true)
            .interact_text_on(&term)?;
        if tag.is_empty() {
            break;
        }
        tags.push(tag);
        tags.sort();
        term.clear_last_lines(2)?;
    }

    // Get Language
    let mut language: Option<String> = Some(
        Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Project Language")
            .with_initial_text(language.unwrap_or_default())
            .allow_empty(true)
            .interact_text()?,
    );
    // Set to none if empty
    if let Some(lang) = &language {
        if lang.is_empty() {
            language = None;
        }
    };

    // Get Category
    let mut category: Option<String> = Some(
        Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Project Category")
            .with_initial_text(category.unwrap_or_default())
            .allow_empty(true)
            .interact_text()?,
    );

    // Get to None if Empty
    if let Some(d) = &category {
        if d.is_empty() {
            category = None;
        }
    };

    Ok(NewParams {
        name,
        desc,
        tags,
        language,
        category,
    })
}
