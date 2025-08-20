use dotenv::from_filename;
use reqwest::Method;

use crate::{
    client::HarborClient,
    request::{
        HarborRequest,
        v2::{
            health::get::GetHealth,
            project::{
                get::{GetProjectArtifacts, GetProjectSummary, GetProjects},
                head::HeadProjects,
            },
            search::get::GetSearch,
            statistics::get::GetStatistics,
        },
    },
};

/// Name of the project to be used in below integration tests
const PROJECT_NAME: &str = "<PROJECT_NAME>";

#[test]
fn harbor_client_can_be_initialized_with_different_credentials() {
    // Load the template file, containing "OTHER_USER" and "OTHER_PASS"
    from_filename(".env.template").ok();

    // Create the client from (a different) environment file
    let client = HarborClient::from_env("OTHER_USER", "OTHER_PASS").unwrap();

    // Assert the values from .env.template are being applied
    assert_eq!("<OTHER_USERNAME>", client.username);
    assert_eq!("<OTHER_PASSWORD>", client.password);
}

#[tokio::test]
async fn get_projects_from_workspace() {
    let request = GetProjects::builder().page_size(50).build().unwrap();
    let projects = test_get(request).await;
    assert!(!projects.is_empty());
}

#[tokio::test]
async fn get_project_summary() {
    let request = GetProjectSummary::builder(PROJECT_NAME)
        .request_id("SomeID")
        .is_resource_name(true)
        .build()
        .unwrap();
    let _project_summary = test_get(request).await;
}

#[tokio::test]
async fn get_project_artifacts() {
    let request = GetProjectArtifacts::builder(PROJECT_NAME).build().unwrap();
    let artifacts = test_get(request).await;
    assert!(!artifacts.is_empty());
}

#[tokio::test]
async fn head_projects() {
    let request = HeadProjects::builder(PROJECT_NAME).build().unwrap();
    let _ = test_head(request).await;
}

#[tokio::test]
async fn get_health() {
    let request = GetHealth::builder().build().unwrap();
    let _ = test_get(request).await;
}

#[tokio::test]
async fn get_search() {
    let request = GetSearch::builder(PROJECT_NAME).build().unwrap();
    let _ = test_get(request).await;
}

#[tokio::test]
async fn get_statistics() {
    let request = GetStatistics::builder().build().unwrap();
    let _ = test_get(request).await;
}

async fn test_get<R: HarborRequest>(request: R) -> R::Response {
    test_request(request, Method::GET).await
}

async fn test_head<R: HarborRequest>(request: R) -> R::Response {
    test_request(request, Method::HEAD).await
}

async fn test_request<R: HarborRequest>(request: R, method: Method) -> R::Response {
    // Initialize a default client (using valid .env credentials)
    let client = HarborClient::default();
    println!("1");
    // Send the request and deserialize the response
    let response = match method {
        Method::DELETE => client.delete(request).await,
        Method::GET => client.get(request).await,
        Method::HEAD => client.head(request).await,
        Method::PATCH => client.patch(request).await,
        Method::POST => client.post(request).await,
        Method::PUT => client.put(request).await,
        _ => panic!("Unsupported method: {method}"),
    };

    // Print the response
    println!("{response:?}");

    // Verify the response is correct
    assert!(response.is_ok());

    // Return unwrapped response
    response.unwrap()
}
