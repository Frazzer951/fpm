use crate::utils::Result;
use clap::ArgMatches;
use console::Term;
use dialoguer::theme::ColorfulTheme;
use dialoguer::Input;
use lib_fpm::{config::Config, database::add_project, project::Project};
use std::path::PathBuf;

struct AddParams {
    pub(crate) name: Option<String>,
    pub(crate) dir: Option<PathBuf>,
    pub(crate) desc: Option<String>,
    pub(crate) tags: Vec<String>,
    pub(crate) language: Option<String>,
    pub(crate) category: Option<String>,
}

pub fn add(sub_matches: &ArgMatches, config: &Config) -> Result<()> {
    let mut name = sub_matches.get_one::<String>("name").cloned();
    let mut dir = sub_matches.get_one::<PathBuf>("directory").cloned();
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
        let new_params = add_params_interactive(name, dir, desc, tags, language, category)?;

        name = new_params.name;
        dir = new_params.dir;
        desc = new_params.desc;
        tags = new_params.tags;
        language = new_params.language;
        category = new_params.category;

        println!("\n\n");
        println!("Name: {name:?}");
        println!("Dir: {dir:?}");
        println!("Desc: {desc:?}");
        println!("Tags: {tags:?}");
        println!("Language: {language:?}");
        println!("Category: {category:?}");
    }
    if name.is_none() {
        println!("A name is required for a project, please specify one");
        return Ok(());
    }
    if dir.is_none() {
        println!("A directory is required for a project, please specify one");
        return Ok(());
    }
    let mut project = Project::new(name, desc, tags, language, category);
    project.directory = dir;
    add_project(config, &project)?;
    println!("{project:#?}");
    Ok(())
}

fn add_params_interactive(
    name: Option<String>,
    dir: Option<PathBuf>,
    desc: Option<String>,
    mut tags: Vec<String>,
    language: Option<String>,
    category: Option<String>,
) -> Result<AddParams> {
    // Get Name
    let name = Some(
        Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Project Name")
            .with_initial_text(name.unwrap_or_default())
            .interact_text()?,
    );

    // Get Directory
    let mut dir_str = dir.unwrap_or_default().to_str().unwrap_or_default().to_owned();
    dir_str = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Project Directory")
        .with_initial_text(dir_str)
        .interact_text()?;
    let dir = Some(PathBuf::from(dir_str));

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

    Ok(AddParams {
        name,
        dir,
        desc,
        tags,
        language,
        category,
    })
}
