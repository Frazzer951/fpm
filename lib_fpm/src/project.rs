use std::path::PathBuf;
use turbosql::Turbosql;

#[derive(Turbosql, Default, Debug, PartialEq, Eq, Clone)]
pub struct Project {
    pub rowid: Option<i64>,
    pub name: Option<String>,
    pub desc: Option<String>,
    pub directory: Option<PathBuf>,
    pub tags: Option<Vec<String>>,
    pub language: Option<String>,
    pub category: Option<String>,
}

impl Project {}
