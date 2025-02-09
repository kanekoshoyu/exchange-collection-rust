#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// StopOrderMessage represents a StopOrderMessage model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct StopOrderMessage {
    #[serde(rename="type")]
    pub reserved_type: String,
    #[serde(rename="product_id")]
    pub product_id: String,
    #[serde(rename="timestamp")]
    pub timestamp: String,
    #[serde(rename="user_id")]
    pub user_id: String,
    #[serde(rename="profile_id")]
    pub profile_id: String,
    #[serde(rename="order_id")]
    pub order_id: String,
    #[serde(rename="stop_type")]
    pub stop_type: String,
    #[serde(rename="side")]
    pub side: String,
    #[serde(rename="stop_price")]
    pub stop_price: String,
    #[serde(rename="size")]
    pub size: String,
    #[serde(rename="funds")]
    pub funds: String,
    #[serde(rename="private")]
    pub private: bool,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

