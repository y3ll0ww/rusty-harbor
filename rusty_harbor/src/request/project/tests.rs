use crate::request::{project::get::{
    Project, ProjectArtifacts, ProjectDeletable, ProjectSummary, Projects
}, ToUrl};

const PROJECT_NAME: &str = "some-project-name";

#[test]
fn get_project_request() {
    let request = Project {
        project_name_or_id: PROJECT_NAME.to_string(),
    };
    let expected_url_encoded = format!("projects/{PROJECT_NAME}");
    assert_eq!(expected_url_encoded, request.to_url())
}

#[test]
fn get_project_deletable_request() {
    let request = ProjectDeletable {
        project_name_or_id: PROJECT_NAME.to_string(),
    };
    let expected_url_encoded = format!("projects/{PROJECT_NAME}/_deletable");
    assert_eq!(expected_url_encoded, request.to_url())
}

#[test]
fn get_project_summary_request() {
    let request = ProjectSummary {
        project_name_or_id: PROJECT_NAME.to_string(),
    };
    let expected_url_encoded = format!("projects/{PROJECT_NAME}/summary");
    assert_eq!(expected_url_encoded, request.to_url())
}

#[test]
fn get_project_artifacts() {
    let request = ProjectArtifacts {
        project_name_or_id: PROJECT_NAME.to_string(),
        with_scan_overview: Some(true),
        page: Some(10),
        ..Default::default()
    };
    let expected_url_encoded =
        format!("projects/{PROJECT_NAME}/artifacts?page=10&with_scan_overview=true");
    assert_eq!(expected_url_encoded, request.to_url())
}

#[test]
fn get_project_artifacts_with_builder() {
    let request = ProjectArtifacts::new(PROJECT_NAME)
        .page(10)
        .with_scan_overview(true)
        .build()
        .unwrap();

    let expected_url_encoded =
        format!("projects/{PROJECT_NAME}/artifacts?page=10&with_scan_overview=true");
    assert_eq!(expected_url_encoded, request.to_url())
}

#[test]
fn get_projects_request() {
    let request = Projects {
        page: Some(10),
        page_size: Some(100),
        name: Some("foobar".to_string()),
        public: Some(false),
        ..Default::default()
    };
    let expected_url_encoded = "projects?page=10&page_size=100&name=foobar&public=false";
    assert_eq!(expected_url_encoded, request.to_url())
}

#[test]
fn get_projects_request_with_builder() {
    let request = Projects::new()
        .page(10)
        .page_size(100)
        .name("foobar")
        .public(false)
        .build()
        .unwrap();
    let expected_url_encoded = "projects?page=10&page_size=100&name=foobar&public=false";
    assert_eq!(expected_url_encoded, request.to_url())
}
