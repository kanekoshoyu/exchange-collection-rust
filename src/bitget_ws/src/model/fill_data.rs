#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// FillData represents a FillData model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct FillData {
    #[serde(rename="orderId")]
    pub order_id: String,
    #[serde(rename="tradeId")]
    pub trade_id: String,
    #[serde(rename="symbol")]
    pub symbol: String,
    #[serde(rename="orderType")]
    pub order_type: String,
    #[serde(rename="side")]
    pub side: String,
    #[serde(rename="priceAvg")]
    pub price_avg: String,
    #[serde(rename="size")]
    pub size: String,
    #[serde(rename="amount")]
    pub amount: String,
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

