#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// OrderChannelPushData represents a OrderChannelPushData model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct OrderChannelPushData {
    #[serde(rename="sprdId", skip_serializing_if = "Option::is_none")]
    pub sprd_id: Option<String>,
    #[serde(rename="ordId", skip_serializing_if = "Option::is_none")]
    pub ord_id: Option<String>,
    #[serde(rename="clOrdId", skip_serializing_if = "Option::is_none")]
    pub cl_ord_id: Option<String>,
    #[serde(rename="tag", skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    #[serde(rename="px", skip_serializing_if = "Option::is_none")]
    pub px: Option<String>,
    #[serde(rename="sz", skip_serializing_if = "Option::is_none")]
    pub sz: Option<String>,
    #[serde(rename="ordType", skip_serializing_if = "Option::is_none")]
    pub ord_type: Option<String>,
    #[serde(rename="side", skip_serializing_if = "Option::is_none")]
    pub side: Option<String>,
    #[serde(rename="fillSz", skip_serializing_if = "Option::is_none")]
    pub fill_sz: Option<String>,
    #[serde(rename="fillPx", skip_serializing_if = "Option::is_none")]
    pub fill_px: Option<String>,
    #[serde(rename="tradeId", skip_serializing_if = "Option::is_none")]
    pub trade_id: Option<String>,
    #[serde(rename="accFillSz", skip_serializing_if = "Option::is_none")]
    pub acc_fill_sz: Option<String>,
    #[serde(rename="pendingFillSz", skip_serializing_if = "Option::is_none")]
    pub pending_fill_sz: Option<String>,
    #[serde(rename="pendingSettleSz", skip_serializing_if = "Option::is_none")]
    pub pending_settle_sz: Option<String>,
    #[serde(rename="canceledSz", skip_serializing_if = "Option::is_none")]
    pub canceled_sz: Option<String>,
    #[serde(rename="avgPx", skip_serializing_if = "Option::is_none")]
    pub avg_px: Option<String>,
    #[serde(rename="state", skip_serializing_if = "Option::is_none")]
    pub state: Option<Box<StateEnum>>,
    #[serde(rename="cancelSource", skip_serializing_if = "Option::is_none")]
    pub cancel_source: Option<String>,
    #[serde(rename="uTime", skip_serializing_if = "Option::is_none")]
    pub u_time: Option<String>,
    #[serde(rename="cTime", skip_serializing_if = "Option::is_none")]
    pub c_time: Option<String>,
    #[serde(rename="code", skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(rename="msg", skip_serializing_if = "Option::is_none")]
    pub msg: Option<String>,
    #[serde(rename="reqId", skip_serializing_if = "Option::is_none")]
    pub req_id: Option<String>,
    #[serde(rename="amendResult", skip_serializing_if = "Option::is_none")]
    pub amend_result: Option<String>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

