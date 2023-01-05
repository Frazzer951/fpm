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

impl Project {
    pub fn new(
        name: Option<String>,
        desc: Option<String>,
        tags: Vec<String>,
        language: Option<String>,
        category: Option<String>,
    ) -> Self {
        Project {
            rowid: None,
            name,
            desc,
            directory: None,
            tags: Some(tags),
            language,
            category,
        }
    }
}
