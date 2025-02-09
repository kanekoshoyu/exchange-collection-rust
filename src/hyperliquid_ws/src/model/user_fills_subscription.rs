#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// UserFillsSubscription represents a UserFillsSubscription model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct UserFillsSubscription {
    #[serde(rename="type")]
    pub reserved_type: Box<UserFillsEnum>,
    #[serde(rename="user")]
    pub user: String,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

