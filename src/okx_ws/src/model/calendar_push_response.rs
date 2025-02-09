#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// CalendarPushResponse represents a CalendarPushResponse model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct CalendarPushResponse {
    #[serde(rename="channel", skip_serializing_if = "Option::is_none")]
    pub channel: Option<String>,
    #[serde(rename="data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<CalendarPushData>>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

