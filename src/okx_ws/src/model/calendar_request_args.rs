#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// CalendarRequestArgs represents a CalendarRequestArgs model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct CalendarRequestArgs {
    #[serde(rename="channel", skip_serializing_if = "Option::is_none")]
    pub channel: Option<String>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

