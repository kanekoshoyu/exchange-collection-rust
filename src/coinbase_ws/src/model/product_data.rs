#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// ProductData represents a ProductData model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct ProductData {
    #[serde(rename="id")]
    pub id: String,
    #[serde(rename="base_currency")]
    pub base_currency: String,
    #[serde(rename="quote_currency")]
    pub quote_currency: String,
    #[serde(rename="base_increment")]
    pub base_increment: String,
    #[serde(rename="quote_increment")]
    pub quote_increment: String,
    #[serde(rename="display_name")]
    pub display_name: String,
    #[serde(rename="status")]
    pub status: String,
    #[serde(rename="status_message", skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
    #[serde(rename="min_market_funds")]
    pub min_market_funds: String,
    #[serde(rename="post_only")]
    pub post_only: bool,
    #[serde(rename="limit_only")]
    pub limit_only: bool,
    #[serde(rename="cancel_only")]
    pub cancel_only: bool,
    #[serde(rename="fx_stablecoin")]
    pub fx_stablecoin: bool,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

