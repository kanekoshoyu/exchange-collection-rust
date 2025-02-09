#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// TradeSubscriptionPayload represents a TradeSubscriptionPayload model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct TradeSubscriptionPayload {
    #[serde(rename="type")]
    pub reserved_type: Box<SubscriptionEnum>,
    #[serde(rename="product_ids")]
    pub product_ids: Vec<String>,
    #[serde(rename="channel")]
    pub channel: Box<TradeChannelEnum>,
    #[serde(rename="jwt")]
    pub jwt: String,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

