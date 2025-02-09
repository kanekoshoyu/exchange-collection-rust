#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// TradeData2 represents a TradeData2 model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct TradeData2 {
    #[serde(rename="instId", skip_serializing_if = "Option::is_none")]
    pub inst_id: Option<String>,
    #[serde(rename="tradeId", skip_serializing_if = "Option::is_none")]
    pub trade_id: Option<String>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

