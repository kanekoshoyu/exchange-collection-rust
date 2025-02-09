#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// NotificationsSnapshotResponse represents a NotificationsSnapshotResponse model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct NotificationsSnapshotResponse {
    #[serde(rename="feed")]
    pub feed: String,
    #[serde(rename="notifications")]
    pub notifications: Vec<Notification>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

