use std::path::PathBuf;

pub struct Project {
    pub name: String,
    pub desc: String,
    pub directory: PathBuf,
    pub tags: Vec<String>,
    pub language: Option<String>,
    pub category: Option<String>,
}

impl Project {}
