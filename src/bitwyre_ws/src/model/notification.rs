#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// Notification represents a Notification model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Notification {
    #[serde(rename="id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename="user_uuid", skip_serializing_if = "Option::is_none")]
    pub user_uuid: Option<String>,
    #[serde(rename="notification_type", skip_serializing_if = "Option::is_none")]
    pub notification_type: Option<i32>,
    #[serde(rename="notification_message", skip_serializing_if = "Option::is_none")]
    pub notification_message: Option<String>,
    #[serde(rename="timestamp", skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<i32>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

