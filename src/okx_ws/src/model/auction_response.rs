#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// AuctionResponse represents a AuctionResponse model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct AuctionResponse {
    #[serde(rename="arg", skip_serializing_if = "Option::is_none")]
    pub arg: Option<Box<InstArgs>>,
    #[serde(rename="data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<AuctionData>>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

