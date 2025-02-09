#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// TickerLiteSnapshotResponse represents a TickerLiteSnapshotResponse model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct TickerLiteSnapshotResponse {
    #[serde(rename="feed")]
    pub feed: String,
    #[serde(rename="product_id")]
    pub product_id: String,
    #[serde(rename="bid")]
    pub bid: f64,
    #[serde(rename="ask")]
    pub ask: f64,
    #[serde(rename="change", skip_serializing_if = "Option::is_none")]
    pub change: Option<f64>,
    #[serde(rename="premium", skip_serializing_if = "Option::is_none")]
    pub premium: Option<f64>,
    #[serde(rename="volume")]
    pub volume: f64,
    #[serde(rename="tag", skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    #[serde(rename="pair", skip_serializing_if = "Option::is_none")]
    pub pair: Option<String>,
    #[serde(rename="dtm", skip_serializing_if = "Option::is_none")]
    pub dtm: Option<i32>,
    #[serde(rename="maturityTime", skip_serializing_if = "Option::is_none")]
    pub maturity_time: Option<i32>,
    #[serde(rename="volumeQuote", skip_serializing_if = "Option::is_none")]
    pub volume_quote: Option<f64>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

