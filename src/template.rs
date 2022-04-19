use serde::Deserialize;

pub fn load_template(template_name: &str) -> Result<Template, Box<dyn std::error::Error>> {
    let filename = format!("templates/{template_name}.json");
    let f = std::fs::File::open(filename)?;
    let d: Template = serde_json::from_reader(f)?;
    Ok(d)
}

#[derive(Debug, Deserialize)]
pub struct Template {
    pub folders: Option<Vec<Folder>>,
    pub files: Option<Vec<File>>,
    pub commands: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct Folder {
    pub name: String,
    pub files: Option<Vec<File>>,
    pub sub_folders: Option<Vec<Folder>>,
}

#[derive(Debug, Deserialize)]
pub struct File {
    pub name: String,
    pub lines: Vec<String>,
}

impl Folder {
    pub fn add_sub_folder(&mut self, folder: Folder) {
        self.sub_folders.get_or_insert_with(Vec::new).push(folder);
    }
    pub fn add_file(&mut self, file: File) {
        self.files.get_or_insert_with(Vec::new).push(file);
    }
}
