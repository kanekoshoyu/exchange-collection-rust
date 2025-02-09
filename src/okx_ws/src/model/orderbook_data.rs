#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// OrderbookData represents a OrderbookData model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct OrderbookData {
    #[serde(rename="asks", skip_serializing_if = "Option::is_none")]
    pub asks: Option<Vec<Vec<String>>>,
    #[serde(rename="bids", skip_serializing_if = "Option::is_none")]
    pub bids: Option<Vec<Vec<String>>>,
    #[serde(rename="ts", skip_serializing_if = "Option::is_none")]
    pub ts: Option<String>,
    #[serde(rename="checksum", skip_serializing_if = "Option::is_none")]
    pub checksum: Option<i32>,
    #[serde(rename="prevSeqId", skip_serializing_if = "Option::is_none")]
    pub prev_seq_id: Option<i32>,
    #[serde(rename="seqId", skip_serializing_if = "Option::is_none")]
    pub seq_id: Option<i32>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

