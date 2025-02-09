#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// WebData2Subscription represents a WebData2Subscription model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct WebData2Subscription {
    #[serde(rename="type")]
    pub reserved_type: Box<WebDataEnum>,
    #[serde(rename="user")]
    pub user: String,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

