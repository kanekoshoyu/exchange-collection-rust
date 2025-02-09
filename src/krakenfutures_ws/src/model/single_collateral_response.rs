#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// SingleCollateralResponse represents a SingleCollateralResponse model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct SingleCollateralResponse {
    #[serde(rename="feed")]
    pub feed: String,
    #[serde(rename="account")]
    pub account: String,
    #[serde(rename="holding", skip_serializing_if = "Option::is_none")]
    pub holding: Option<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename="futures")]
    pub futures: std::collections::HashMap<String, AnonymousSchema101>,
    #[serde(rename="timestamp")]
    pub timestamp: i64,
    #[serde(rename="seq")]
    pub seq: i64,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

