#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// OpenOrdersSnapshot represents a OpenOrdersSnapshot model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct OpenOrdersSnapshot {
    #[serde(rename="feed")]
    pub feed: Box<FeedEnum>,
    #[serde(rename="account")]
    pub account: String,
    #[serde(rename="orders")]
    pub orders: Vec<Order>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

