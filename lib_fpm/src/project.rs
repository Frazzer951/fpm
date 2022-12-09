use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Project {
    pub name: String,
    pub desc: String,
    pub directory: PathBuf,
    pub tags: Vec<String>,
    pub language: Option<String>,
    pub category: Option<String>,
}

impl Project {}
