use serde::Deserialize;

/// Check the status of Harbor components. This path does not require authentication.
/// Response to the [`GetHealth`](crate::request::v2::health::get::GetHealth) request.
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
