use crate::{client::HarborClient, request::project, response::project::Project};

#[test]
fn new_client() {
    let client = HarborClient::from_env("ROBOT_USER_1", "ROBOT_PASS_1").unwrap();
    println!("{}", client.username);
    println!("{}", client.password);

    let client = HarborClient::default();
    println!("{}", client.username);
    println!("{}", client.password);
}

#[tokio::test]
async fn foo_bar() {
    let client = HarborClient::default();

    let request = project::get::Projects::builder()
        .page_size(50)
        .build()
        .unwrap();

    let projects = client.get::<_, Vec<Project>>(request).await;
    assert!(projects.is_ok());
    println!("{:?}", projects.unwrap());
}
