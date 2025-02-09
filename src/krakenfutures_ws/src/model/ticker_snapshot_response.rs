#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// TickerSnapshotResponse represents a TickerSnapshotResponse model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct TickerSnapshotResponse {
    #[serde(rename="time")]
    pub time: i32,
    #[serde(rename="product_id")]
    pub product_id: String,
    #[serde(rename="funding_rate", skip_serializing_if = "Option::is_none")]
    pub funding_rate: Option<f64>,
    #[serde(rename="funding_rate_prediction", skip_serializing_if = "Option::is_none")]
    pub funding_rate_prediction: Option<f64>,
    #[serde(rename="relative_funding_rate", skip_serializing_if = "Option::is_none")]
    pub relative_funding_rate: Option<f64>,
    #[serde(rename="relative_funding_rate_prediction", skip_serializing_if = "Option::is_none")]
    pub relative_funding_rate_prediction: Option<f64>,
    #[serde(rename="next_funding_rate_time", skip_serializing_if = "Option::is_none")]
    pub next_funding_rate_time: Option<i32>,
    #[serde(rename="feed")]
    pub feed: String,
    #[serde(rename="bid")]
    pub bid: f64,
    #[serde(rename="ask")]
    pub ask: f64,
    #[serde(rename="bid_size")]
    pub bid_size: f64,
    #[serde(rename="ask_size")]
    pub ask_size: f64,
    #[serde(rename="volume")]
    pub volume: f64,
    #[serde(rename="dtm", skip_serializing_if = "Option::is_none")]
    pub dtm: Option<i32>,
    #[serde(rename="leverage", skip_serializing_if = "Option::is_none")]
    pub leverage: Option<String>,
    #[serde(rename="index", skip_serializing_if = "Option::is_none")]
    pub index: Option<f64>,
    #[serde(rename="change", skip_serializing_if = "Option::is_none")]
    pub change: Option<f64>,
    #[serde(rename="suspended", skip_serializing_if = "Option::is_none")]
    pub suspended: Option<bool>,
    #[serde(rename="tag", skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    #[serde(rename="pair", skip_serializing_if = "Option::is_none")]
    pub pair: Option<String>,
    #[serde(rename="openInterest", skip_serializing_if = "Option::is_none")]
    pub open_interest: Option<f64>,
    #[serde(rename="markPrice", skip_serializing_if = "Option::is_none")]
    pub mark_price: Option<f64>,
    #[serde(rename="maturityTime", skip_serializing_if = "Option::is_none")]
    pub maturity_time: Option<i32>,
    #[serde(rename="post_only", skip_serializing_if = "Option::is_none")]
    pub post_only: Option<bool>,
    #[serde(rename="volumeQuote", skip_serializing_if = "Option::is_none")]
    pub volume_quote: Option<f64>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

