#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// SubscriptionPayload represents a SubscriptionPayload model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct SubscriptionPayload {
    #[serde(rename="name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename="type", skip_serializing_if = "Option::is_none")]
    pub reserved_type: Option<Box<SubscriptionEnum>>,
    #[serde(rename="product_ids", skip_serializing_if = "Option::is_none")]
    pub product_ids: Option<Vec<String>>,
    #[serde(rename="channels", skip_serializing_if = "Option::is_none")]
    pub channels: Option<Vec<ChannelOrProductIds>>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

