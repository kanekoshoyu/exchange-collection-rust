#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// CalendarRequest represents a CalendarRequest model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct CalendarRequest {
    #[serde(rename="op", skip_serializing_if = "Option::is_none")]
    pub op: Option<String>,
    #[serde(rename="args", skip_serializing_if = "Option::is_none")]
    pub args: Option<Vec<CalendarRequestArgs>>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

