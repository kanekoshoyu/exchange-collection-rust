#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// MatchMakerMessage represents a MatchMakerMessage model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct MatchMakerMessage {
    #[serde(rename="type")]
    pub reserved_type: String,
    #[serde(rename="trade_id")]
    pub trade_id: i32,
    #[serde(rename="sequence")]
    pub sequence: i32,
    #[serde(rename="maker_order_id")]
    pub maker_order_id: String,
    #[serde(rename="taker_order_id")]
    pub taker_order_id: String,
    #[serde(rename="time")]
    pub time: String,
    #[serde(rename="product_id")]
    pub product_id: String,
    #[serde(rename="size")]
    pub size: String,
    #[serde(rename="price")]
    pub price: String,
    #[serde(rename="side")]
    pub side: String,
    #[serde(rename="maker_user_id")]
    pub maker_user_id: String,
    #[serde(rename="user_id")]
    pub user_id: String,
    #[serde(rename="maker_profile_id")]
    pub maker_profile_id: String,
    #[serde(rename="profile_id")]
    pub profile_id: String,
    #[serde(rename="maker_fee_rate")]
    pub maker_fee_rate: String,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

