#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// Request represents a Request model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Request {
    #[serde(rename="type", skip_serializing_if = "Option::is_none")]
    pub reserved_type: Option<String>,
    #[serde(rename="channel", skip_serializing_if = "Option::is_none")]
    pub channel: Option<String>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

