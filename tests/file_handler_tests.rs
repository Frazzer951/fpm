use fpm::file_handler::{FileError, Project, Projects};
use fs_err as fs;

mod common;

#[test]
fn test_projects_load() {
    let mut test_dir = common::get_test_dir();
    test_dir.push("example_files");
    test_dir.push("projects_db.json");

    let projects = Projects::load(test_dir.clone()).unwrap();
    assert_eq!(
        projects,
        Projects {
            projects:     vec![Project {
                name:      "test_proj".to_string(),
                directory: "/home/frazzer/dev/test_proj".to_string(),
                category:  None,
                p_type:    None,
            }],
            project_path: test_dir,
        }
    )
}

#[test]
fn test_projects_load_no_file() {
    let mut test_dir = common::get_test_dir();
    test_dir.push("projects_db.json");

    let projects = Projects::load(test_dir.clone()).unwrap_err();

    assert_eq!(projects, FileError::LoadingError);
}

#[test]
fn test_projects_load_parse_error_1() {
    let mut test_dir = common::get_test_dir();
    test_dir.push("example_files");
    test_dir.push("projects_db_parse_error_1.json");

    let projects = Projects::load(test_dir.clone()).unwrap_err();

    assert_eq!(projects, FileError::ParsingError);
}

#[test]
fn test_projects_load_parse_error_2() {
    let mut test_dir = common::get_test_dir();
    test_dir.push("example_files");
    test_dir.push("projects_db_parse_error_2.json");

    let projects = Projects::load(test_dir.clone()).unwrap_err();

    assert_eq!(projects, FileError::ParsingError);
}

#[test]
fn test_projects_save() {
    let mut projects_db_path = common::get_test_dir();
    projects_db_path.push("projects_db_save.json");

    let mut projects = Projects {
        project_path: projects_db_path.clone(),
        ..Projects::default(None)
    };
    projects.projects.push(Project {
        name:      "test_proj".to_string(),
        directory: "/home/frazzer/dev/test_proj".to_string(),
        category:  None,
        p_type:    None,
    });

    projects.save();

    let mut expected_projects_db_path = common::get_test_dir();
    expected_projects_db_path.push("example_files");
    expected_projects_db_path.push("projects_db.json");

    let mut expected_projects_db = fs::read_to_string(expected_projects_db_path).unwrap();
    expected_projects_db = expected_projects_db.replace('\n', "");
    let projects_db = fs::read_to_string(&projects_db_path).unwrap();

    assert_eq!(expected_projects_db, projects_db);

    // delete config.toml if it exists
    if projects_db_path.exists() {
        fs::remove_file(&projects_db_path).unwrap();
    }
}
