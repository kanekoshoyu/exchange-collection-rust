#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// TradeChannelPushResponse represents a TradeChannelPushResponse model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct TradeChannelPushResponse {
    #[serde(rename="arg", skip_serializing_if = "Option::is_none")]
    pub arg: Option<Box<OrderChannelDetails>>,
    #[serde(rename="data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<TradeChannelPushData>>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

