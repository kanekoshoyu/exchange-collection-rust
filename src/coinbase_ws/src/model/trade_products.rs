#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// TradeProducts represents a TradeProducts model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct TradeProducts {
    #[serde(rename="product_type", skip_serializing_if = "Option::is_none")]
    pub product_type: Option<String>,
    #[serde(rename="id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename="base_currency", skip_serializing_if = "Option::is_none")]
    pub base_currency: Option<String>,
    #[serde(rename="quote_currency", skip_serializing_if = "Option::is_none")]
    pub quote_currency: Option<String>,
    #[serde(rename="base_increment", skip_serializing_if = "Option::is_none")]
    pub base_increment: Option<String>,
    #[serde(rename="quote_increment", skip_serializing_if = "Option::is_none")]
    pub quote_increment: Option<String>,
    #[serde(rename="display_name", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename="status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename="status_message", skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
    #[serde(rename="min_market_funds", skip_serializing_if = "Option::is_none")]
    pub min_market_funds: Option<String>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

