#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// FutureData represents a FutureData model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct FutureData {
    #[serde(rename="instId")]
    pub inst_id: String,
    #[serde(rename="lastPr")]
    pub last_pr: String,
    #[serde(rename="bidPr")]
    pub bid_pr: String,
    #[serde(rename="askPr")]
    pub ask_pr: String,
    #[serde(rename="bidSz")]
    pub bid_sz: String,
    #[serde(rename="askSz")]
    pub ask_sz: String,
    #[serde(rename="open24h")]
    pub open24h: String,
    #[serde(rename="high24h")]
    pub high24h: String,
    #[serde(rename="low24h")]
    pub low24h: String,
    #[serde(rename="change24h")]
    pub change24h: String,
    #[serde(rename="fundingRate")]
    pub funding_rate: String,
    #[serde(rename="nextFundingTime")]
    pub next_funding_time: String,
    #[serde(rename="markPrice")]
    pub mark_price: String,
    #[serde(rename="indexPrice")]
    pub index_price: String,
    #[serde(rename="holdingAmount")]
    pub holding_amount: String,
    #[serde(rename="baseVolume")]
    pub base_volume: String,
    #[serde(rename="quoteVolume")]
    pub quote_volume: String,
    #[serde(rename="openUtc")]
    pub open_utc: String,
    #[serde(rename="symbolType")]
    pub symbol_type: i32,
    #[serde(rename="symbol")]
    pub symbol: String,
    #[serde(rename="deliveryPrice")]
    pub delivery_price: String,
    #[serde(rename="ts")]
    pub ts: String,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

