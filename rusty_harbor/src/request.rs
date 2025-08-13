pub mod project;

pub trait HarborRequest {
    fn to_url(&self) -> String;
}
