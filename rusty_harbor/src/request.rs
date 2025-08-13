use serde::de::DeserializeOwned;

pub mod project;

pub trait HarborRequest {
    type Response: DeserializeOwned;
    fn to_url(&self) -> String;
}
