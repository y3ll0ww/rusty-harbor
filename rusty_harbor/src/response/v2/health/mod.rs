use serde::Deserialize;
//The system health status
/// Response to requests that request project(s):
/// - [`GetProject`](crate::request::project::get::GetProject)
/// - [`GetProjects`](crate::request::project::get::GetProjects) (in a vector)
#[derive(Debug, Deserialize)]
pub struct OverallHealthStatus {
    /// The overall health status. It is "healthy" only when all the components' status are
    /// "healthy".
    pub status: String,
    /// The health status of components
    pub components: Vec<ComponentHealthStatus>,
}

/// The health status of component
#[derive(Debug, Deserialize)]
pub struct ComponentHealthStatus {
    /// The component name
    pub name: String,
    /// The health status of component. Is either "healthy" or "unhealthy".
    pub status: String,
    /// (optional) The error message when the status is "unhealthy".
    pub error: Option<String>,
}
