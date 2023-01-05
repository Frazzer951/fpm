use crate::utils::Result;
use console::Term;
use dialoguer::theme::ColorfulTheme;
use dialoguer::Input;

pub(crate) struct NewParams {
    pub(crate) name: Option<String>,
    pub(crate) desc: Option<String>,
    pub(crate) tags: Vec<String>,
    pub(crate) language: Option<String>,
    pub(crate) category: Option<String>,
}

pub(crate) fn new_params_interactive(
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
