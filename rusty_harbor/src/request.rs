pub mod project;

pub trait ToUrl {
    fn to_url(&self) -> String;
}
