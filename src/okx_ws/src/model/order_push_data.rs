#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// OrderPushData represents a OrderPushData model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct OrderPushData {
    #[serde(rename="instType", skip_serializing_if = "Option::is_none")]
    pub inst_type: Option<String>,
    #[serde(rename="instId", skip_serializing_if = "Option::is_none")]
    pub inst_id: Option<String>,
    #[serde(rename="tgtCcy", skip_serializing_if = "Option::is_none")]
    pub tgt_ccy: Option<String>,
    #[serde(rename="ccy", skip_serializing_if = "Option::is_none")]
    pub ccy: Option<String>,
    #[serde(rename="ordId", skip_serializing_if = "Option::is_none")]
    pub ord_id: Option<String>,
    #[serde(rename="clOrdId", skip_serializing_if = "Option::is_none")]
    pub cl_ord_id: Option<String>,
    #[serde(rename="tag", skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    #[serde(rename="px", skip_serializing_if = "Option::is_none")]
    pub px: Option<String>,
    #[serde(rename="pxUsd", skip_serializing_if = "Option::is_none")]
    pub px_usd: Option<String>,
    #[serde(rename="pxVol", skip_serializing_if = "Option::is_none")]
    pub px_vol: Option<String>,
    #[serde(rename="pxType", skip_serializing_if = "Option::is_none")]
    pub px_type: Option<String>,
    #[serde(rename="sz", skip_serializing_if = "Option::is_none")]
    pub sz: Option<String>,
    #[serde(rename="notionalUsd", skip_serializing_if = "Option::is_none")]
    pub notional_usd: Option<String>,
    #[serde(rename="ordType", skip_serializing_if = "Option::is_none")]
    pub ord_type: Option<Box<OrderEnum>>,
    #[serde(rename="side", skip_serializing_if = "Option::is_none")]
    pub side: Option<String>,
    #[serde(rename="posSide", skip_serializing_if = "Option::is_none")]
    pub pos_side: Option<Box<PosEnum>>,
    #[serde(rename="tdMode", skip_serializing_if = "Option::is_none")]
    pub td_mode: Option<String>,
    #[serde(rename="fillPx", skip_serializing_if = "Option::is_none")]
    pub fill_px: Option<String>,
    #[serde(rename="tradeId", skip_serializing_if = "Option::is_none")]
    pub trade_id: Option<String>,
    #[serde(rename="fillSz", skip_serializing_if = "Option::is_none")]
    pub fill_sz: Option<String>,
    #[serde(rename="fillPnl", skip_serializing_if = "Option::is_none")]
    pub fill_pnl: Option<String>,
    #[serde(rename="fillTime", skip_serializing_if = "Option::is_none")]
    pub fill_time: Option<String>,
    #[serde(rename="state", skip_serializing_if = "Option::is_none")]
    pub state: Option<Box<StateEnum>>,
    #[serde(rename="lever", skip_serializing_if = "Option::is_none")]
    pub lever: Option<String>,
    #[serde(rename="uTime", skip_serializing_if = "Option::is_none")]
    pub u_time: Option<String>,
    #[serde(rename="cTime", skip_serializing_if = "Option::is_none")]
    pub c_time: Option<String>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

