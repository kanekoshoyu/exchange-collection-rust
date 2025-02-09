#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// FuturesFillData represents a FuturesFillData model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct FuturesFillData {
    #[serde(rename="orderId")]
    pub order_id: String,
    #[serde(rename="tradeId")]
    pub trade_id: String,
    #[serde(rename="symbol")]
    pub symbol: String,
    #[serde(rename="side")]
    pub side: String,
    #[serde(rename="orderType")]
    pub order_type: String,
    #[serde(rename="posMode")]
    pub pos_mode: String,
    #[serde(rename="price")]
    pub price: String,
    #[serde(rename="baseVolume")]
    pub base_volume: String,
    #[serde(rename="quoteVolume")]
    pub quote_volume: String,
    #[serde(rename="profit")]
    pub profit: String,
    #[serde(rename="tradeSide")]
    pub trade_side: String,
    #[serde(rename="tradeScope")]
    pub trade_scope: String,
    #[serde(rename="feeDetail")]
    pub fee_detail: Vec<FeeDetail>,
    #[serde(rename="cTime")]
    pub c_time: String,
    #[serde(rename="uTime")]
    pub u_time: String,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

