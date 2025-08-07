use crate::{client::HarborClient, request::project, response};

#[test]
fn new_client() {
    let client = HarborClient::from_env("ROBOT_USER_1", "ROBOT_PASS_1").unwrap();
    println!("{}", client.username);
    println!("{}", client.password);

    let client = HarborClient::default().unwrap();
    println!("{}", client.username);
    println!("{}", client.password);
}

#[tokio::test]
async fn foo_bar() {
    let client = HarborClient::default().unwrap();

    let request = project::get::Projects::new().page_size(50).build().unwrap();

    let response = client.get(request).await;
    println!("{response:?}");
    assert!(response.is_ok());

    let text = response.unwrap().text().await.unwrap();
    let deserialized = serde_json::from_str::<Vec<response::project::Project>>(&text);
    println!("{deserialized:?}");
    assert!(deserialized.is_ok());
}
