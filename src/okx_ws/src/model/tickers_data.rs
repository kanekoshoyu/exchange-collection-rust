#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// TickersData represents a TickersData model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct TickersData {
    #[serde(rename="instType", skip_serializing_if = "Option::is_none")]
    pub inst_type: Option<String>,
    #[serde(rename="instId", skip_serializing_if = "Option::is_none")]
    pub inst_id: Option<String>,
    #[serde(rename="last", skip_serializing_if = "Option::is_none")]
    pub last: Option<String>,
    #[serde(rename="lastSz", skip_serializing_if = "Option::is_none")]
    pub last_sz: Option<String>,
    #[serde(rename="askPx", skip_serializing_if = "Option::is_none")]
    pub ask_px: Option<String>,
    #[serde(rename="askSz", skip_serializing_if = "Option::is_none")]
    pub ask_sz: Option<String>,
    #[serde(rename="bidPx", skip_serializing_if = "Option::is_none")]
    pub bid_px: Option<String>,
    #[serde(rename="bidSz", skip_serializing_if = "Option::is_none")]
    pub bid_sz: Option<String>,
    #[serde(rename="open24h", skip_serializing_if = "Option::is_none")]
    pub open24h: Option<String>,
    #[serde(rename="high24h", skip_serializing_if = "Option::is_none")]
    pub high24h: Option<String>,
    #[serde(rename="low24h", skip_serializing_if = "Option::is_none")]
    pub low24h: Option<String>,
    #[serde(rename="volCcy24h", skip_serializing_if = "Option::is_none")]
    pub vol_ccy24h: Option<String>,
    #[serde(rename="vol24h", skip_serializing_if = "Option::is_none")]
    pub vol24h: Option<String>,
    #[serde(rename="sodUtc0", skip_serializing_if = "Option::is_none")]
    pub sod_utc0: Option<String>,
    #[serde(rename="sodUtc8", skip_serializing_if = "Option::is_none")]
    pub sod_utc8: Option<String>,
    #[serde(rename="ts", skip_serializing_if = "Option::is_none")]
    pub ts: Option<String>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

