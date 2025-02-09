#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// OrderData represents a OrderData model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct OrderData {
    #[serde(rename="instId")]
    pub inst_id: String,
    #[serde(rename="orderId")]
    pub order_id: String,
    #[serde(rename="clientOid")]
    pub client_oid: String,
    #[serde(rename="size")]
    pub size: String,
    #[serde(rename="newSize")]
    pub new_size: String,
    #[serde(rename="notional")]
    pub notional: String,
    #[serde(rename="orderType")]
    pub order_type: String,
    #[serde(rename="force")]
    pub force: String,
    #[serde(rename="side")]
    pub side: String,
    #[serde(rename="fillPrice")]
    pub fill_price: String,
    #[serde(rename="tradeId")]
    pub trade_id: String,
    #[serde(rename="baseVolume")]
    pub base_volume: String,
    #[serde(rename="fillTime")]
    pub fill_time: String,
    #[serde(rename="fillFee")]
    pub fill_fee: String,
    #[serde(rename="fillFeeCoin")]
    pub fill_fee_coin: String,
    #[serde(rename="tradeScope")]
    pub trade_scope: String,
    #[serde(rename="accBaseVolume")]
    pub acc_base_volume: String,
    #[serde(rename="priceAvg")]
    pub price_avg: String,
    #[serde(rename="status")]
    pub status: String,
    #[serde(rename="cTime")]
    pub c_time: String,
    #[serde(rename="uTime")]
    pub u_time: String,
    #[serde(rename="stpMode")]
    pub stp_mode: String,
    #[serde(rename="feeDetail")]
    pub fee_detail: Vec<FeeDetail>,
    #[serde(rename="enterPointSource")]
    pub enter_point_source: String,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

