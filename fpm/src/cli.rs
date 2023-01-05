use crate::utils::Result;
use clap::{command, Arg, ArgAction, Command};
use console::Term;
use dialoguer::{theme::ColorfulTheme, Input};
use lib_fpm::project::Project;

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
            .help("The directory to place the project in. If nothing is provided a directory will be generated"),
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

    match matches.subcommand() {
        Some(("new", sub_matches)) => {
            let dir = sub_matches.get_one::<String>("directory").cloned();

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
                name = Some(
                    Input::with_theme(&ColorfulTheme::default())
                        .with_prompt("Project Name")
                        .with_initial_text(name.unwrap_or_default())
                        .interact_text()?,
                );

                desc = Some(
                    Input::with_theme(&ColorfulTheme::default())
                        .with_prompt("Project Desc")
                        .with_initial_text(desc.unwrap_or_default())
                        .allow_empty(true)
                        .interact_text()?,
                );
                if let Some(d) = &desc {
                    if d.is_empty() {
                        desc = None;
                    }
                };

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

                language = Some(
                    Input::with_theme(&ColorfulTheme::default())
                        .with_prompt("Project Language")
                        .with_initial_text(language.unwrap_or_default())
                        .allow_empty(true)
                        .interact_text()?,
                );
                if let Some(lang) = &language {
                    if lang.is_empty() {
                        language = None;
                    }
                };

                category = Some(
                    Input::with_theme(&ColorfulTheme::default())
                        .with_prompt("Project Category")
                        .with_initial_text(category.unwrap_or_default())
                        .allow_empty(true)
                        .interact_text()?,
                );
                if let Some(d) = &category {
                    if d.is_empty() {
                        category = None;
                    }
                };

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

            let _project = Project::new(name, desc, tags, language, category);

            // TODO: Create Project folder
            // TODO: Save Project to database
        },
        _ => unreachable!(),
    }

    Ok(())
}
