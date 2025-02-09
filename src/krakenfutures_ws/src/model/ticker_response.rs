#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// TickerResponse represents a TickerResponse model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct TickerResponse {
    #[serde(rename="event")]
    pub event: Box<EventEnum>,
    #[serde(rename="feed")]
    pub feed: String,
    #[serde(rename="product_ids")]
    pub product_ids: Vec<String>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

