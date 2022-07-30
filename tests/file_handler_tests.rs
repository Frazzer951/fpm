use fpm::file_handler::Projects;

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
            projects:     vec![],
            project_path: test_dir,
        }
    )

    // delete config.toml if it exists
    //if test_dir.exists() {
    //    fs::remove_file(&test_dir).unwrap();
    //}
}
