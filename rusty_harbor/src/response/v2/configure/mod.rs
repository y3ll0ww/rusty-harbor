use std::collections::HashMap;

use serde::Deserialize;
use serde_json::Value;

/// Get system configurations successfully. The response body is a map.
/// Response to the [`GetConfiguration`](crate::request::v2::configure::get::GetConfiguration)
/// request.
pub type InternalConfigurationsResponse = HashMap<String, InternalConfigurationValue>;

#[derive(Debug, Deserialize)]
pub struct InternalConfigurationValue {
    /// The value of current config item.
    pub value: Value,
    /// The configure item can be updated or not.
    pub editable: bool,
}
