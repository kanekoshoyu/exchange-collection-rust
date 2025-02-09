#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// Notification represents a Notification model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Notification {
    #[serde(rename="id")]
    pub id: i64,
    #[serde(rename="type")]
    pub reserved_type: String,
    #[serde(rename="priority")]
    pub priority: String,
    #[serde(rename="note")]
    pub note: String,
    #[serde(rename="effective_time")]
    pub effective_time: i64,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

