use fpm::file_handler::{Project, Projects};
use fpm::project::add_project;
use fs_err as fs;

mod common;

#[test]
fn test_project_add() {
    let mut projects_db_path = common::get_test_dir();
    projects_db_path.push("projects_db_project_add.json");

    let mut projects = Projects {
        project_path: projects_db_path.clone(),
        ..Projects::default(None)
    };

    add_project(
        &mut projects,
        "test_proj".to_string(),
        "./tests".to_string(),
        None,
        None,
    );

    assert_eq!(
        projects.projects[0],
        Project {
            name:      "test_proj".to_string(),
            directory: "./tests".to_string(),
            category:  None,
            p_type:    None,
        }
    );

    // delete config.toml if it exists
    if projects_db_path.exists() {
        fs::remove_file(&projects_db_path).unwrap();
    }
}
