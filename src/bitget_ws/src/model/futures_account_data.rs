#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// FuturesAccountData represents a FuturesAccountData model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct FuturesAccountData {
    #[serde(rename="marginCoin")]
    pub margin_coin: String,
    #[serde(rename="frozen")]
    pub frozen: String,
    #[serde(rename="available")]
    pub available: String,
    #[serde(rename="maxOpenPosAvailable")]
    pub max_open_pos_available: String,
    #[serde(rename="maxTransferOut")]
    pub max_transfer_out: String,
    #[serde(rename="equity")]
    pub equity: String,
    #[serde(rename="usdtEquity")]
    pub usdt_equity: String,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

