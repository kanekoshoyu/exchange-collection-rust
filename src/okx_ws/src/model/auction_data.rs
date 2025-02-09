#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// AuctionData represents a AuctionData model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct AuctionData {
    #[serde(rename="instId", skip_serializing_if = "Option::is_none")]
    pub inst_id: Option<String>,
    #[serde(rename="eqPx", skip_serializing_if = "Option::is_none")]
    pub eq_px: Option<String>,
    #[serde(rename="matchedSz", skip_serializing_if = "Option::is_none")]
    pub matched_sz: Option<String>,
    #[serde(rename="unmatchedSz", skip_serializing_if = "Option::is_none")]
    pub unmatched_sz: Option<String>,
    #[serde(rename="auctionEndTime", skip_serializing_if = "Option::is_none")]
    pub auction_end_time: Option<String>,
    #[serde(rename="state", skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename="ts", skip_serializing_if = "Option::is_none")]
    pub ts: Option<String>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

