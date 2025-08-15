use reqwest::header::HeaderMap;
use serde::de::DeserializeOwned;

pub mod v2;

pub trait HarborRequest {
    type Response: DeserializeOwned + std::fmt::Debug;
    fn to_url(&self) -> String;
    fn headers(&self) -> Result<HeaderMap, String>;
}
