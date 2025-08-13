use std::collections::HashMap;

use serde_json::Value;

use crate::response::v2::types::{AdditionLink, NativeReportSummary};

pub type AdditionLinks = HashMap<String, AdditionLink>;

pub type Annotations = HashMap<String, String>;

pub type ExtraAttrs = HashMap<String, Value>;

pub type ResourceList = HashMap<String, i64>;

/// The scan overview attached in the metadata of tag
pub type ScanOverview = HashMap<String, NativeReportSummary>;

pub type Summary = HashMap<String, usize>;
