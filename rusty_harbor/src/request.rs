use serde::de::DeserializeOwned;

pub mod v2;

pub trait HarborRequest {
    type Response: DeserializeOwned;
    fn to_url(&self) -> String;
}
