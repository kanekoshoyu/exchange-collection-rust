#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// DataItem represents a DataItem model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct DataItem {
    #[serde(rename="instId")]
    pub inst_id: String,
    #[serde(rename="lastPr")]
    pub last_pr: String,
    #[serde(rename="open24h", skip_serializing_if = "Option::is_none")]
    pub open24h: Option<String>,
    #[serde(rename="high24h", skip_serializing_if = "Option::is_none")]
    pub high24h: Option<String>,
    #[serde(rename="low24h", skip_serializing_if = "Option::is_none")]
    pub low24h: Option<String>,
    #[serde(rename="change24h", skip_serializing_if = "Option::is_none")]
    pub change24h: Option<String>,
    #[serde(rename="bidPr", skip_serializing_if = "Option::is_none")]
    pub bid_pr: Option<String>,
    #[serde(rename="askPr", skip_serializing_if = "Option::is_none")]
    pub ask_pr: Option<String>,
    #[serde(rename="bidSz", skip_serializing_if = "Option::is_none")]
    pub bid_sz: Option<String>,
    #[serde(rename="askSz", skip_serializing_if = "Option::is_none")]
    pub ask_sz: Option<String>,
    #[serde(rename="baseVolume", skip_serializing_if = "Option::is_none")]
    pub base_volume: Option<String>,
    #[serde(rename="quoteVolume", skip_serializing_if = "Option::is_none")]
    pub quote_volume: Option<String>,
    #[serde(rename="openUtc", skip_serializing_if = "Option::is_none")]
    pub open_utc: Option<String>,
    #[serde(rename="changeUtc24h", skip_serializing_if = "Option::is_none")]
    pub change_utc24h: Option<String>,
    #[serde(rename="ts")]
    pub ts: String,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

