#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// TradeItems represents a TradeItems model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct TradeItems {
    #[serde(rename="id")]
    pub id: String,
    #[serde(rename="time")]
    pub time: i64,
    #[serde(rename="type")]
    pub reserved_type: Box<TypeEnum>,
    #[serde(rename="price")]
    pub price: String,
    #[serde(rename="volume")]
    pub volume: String,
    #[serde(rename="total")]
    pub total: String,
    #[serde(rename="fee_percentage")]
    pub fee_percentage: String,
    #[serde(rename="fee")]
    pub fee: String,
    #[serde(rename="fee_currency")]
    pub fee_currency: String,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

