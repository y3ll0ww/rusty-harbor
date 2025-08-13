use dotenv::from_filename;

use crate::{client::HarborClient, request::project, response::project::Project};

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
    // Initialize a default client (using valid .env credentials)
    let client = HarborClient::default();

    // Create a GET Projects request through the builder
    let request = project::get::Projects::builder()
        .page_size(50)
        .build()
        .unwrap();

    // Send the request and deserialize the response
    let projects = client.get::<_, Vec<Project>>(request).await;

    // Verify the response is correct
    assert!(projects.is_ok());
}
