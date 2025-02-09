#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// TimeMessagePayload represents a TimeMessagePayload model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct TimeMessagePayload {
    #[serde(rename="unixtime")]
    pub unixtime: i64,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

