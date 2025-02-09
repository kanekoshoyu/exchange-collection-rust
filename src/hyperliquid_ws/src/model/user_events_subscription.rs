#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// UserEventsSubscription represents a UserEventsSubscription model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct UserEventsSubscription {
    #[serde(rename="type")]
    pub reserved_type: Box<EventEnum>,
    #[serde(rename="user")]
    pub user: String,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

