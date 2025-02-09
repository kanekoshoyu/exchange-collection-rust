#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// FillsResponse represents a FillsResponse model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct FillsResponse {
    #[serde(rename="event")]
    pub event: Box<EventEnum>,
    #[serde(rename="feed")]
    pub feed: String,
    #[serde(rename="product_ids")]
    pub product_ids: Vec<String>,
    #[serde(rename="api_key")]
    pub api_key: String,
    #[serde(rename="original_challenge")]
    pub original_challenge: String,
    #[serde(rename="signed_challenge")]
    pub signed_challenge: String,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

