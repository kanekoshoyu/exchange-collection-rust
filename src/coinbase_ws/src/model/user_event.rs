#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// UserEvent represents a UserEvent model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct UserEvent {
    #[serde(rename="type")]
    pub reserved_type: String,
    #[serde(rename="orders")]
    pub orders: Vec<UserOrder>,
    #[serde(rename="positions")]
    pub positions: Box<UserPositions>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

