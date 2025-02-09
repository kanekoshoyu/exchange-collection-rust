#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// GridSuborderData represents a GridSuborderData model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct GridSuborderData {
    #[serde(rename="algoId", skip_serializing_if = "Option::is_none")]
    pub algo_id: Option<String>,
    #[serde(rename="algoClOrdId", skip_serializing_if = "Option::is_none")]
    pub algo_cl_ord_id: Option<String>,
    #[serde(rename="instType", skip_serializing_if = "Option::is_none")]
    pub inst_type: Option<String>,
    #[serde(rename="instId", skip_serializing_if = "Option::is_none")]
    pub inst_id: Option<String>,
    #[serde(rename="algoOrdType", skip_serializing_if = "Option::is_none")]
    pub algo_ord_type: Option<String>,
    #[serde(rename="groupId", skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    #[serde(rename="ordId", skip_serializing_if = "Option::is_none")]
    pub ord_id: Option<String>,
    #[serde(rename="cTime", skip_serializing_if = "Option::is_none")]
    pub c_time: Option<String>,
    #[serde(rename="uTime", skip_serializing_if = "Option::is_none")]
    pub u_time: Option<String>,
    #[serde(rename="tdMode", skip_serializing_if = "Option::is_none")]
    pub td_mode: Option<String>,
    #[serde(rename="tag", skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    #[serde(rename="ordType", skip_serializing_if = "Option::is_none")]
    pub ord_type: Option<String>,
    #[serde(rename="sz", skip_serializing_if = "Option::is_none")]
    pub sz: Option<String>,
    #[serde(rename="state", skip_serializing_if = "Option::is_none")]
    pub state: Option<Box<StateEnum>>,
    #[serde(rename="side", skip_serializing_if = "Option::is_none")]
    pub side: Option<String>,
    #[serde(rename="px", skip_serializing_if = "Option::is_none")]
    pub px: Option<String>,
    #[serde(rename="fee", skip_serializing_if = "Option::is_none")]
    pub fee: Option<String>,
    #[serde(rename="feeCcy", skip_serializing_if = "Option::is_none")]
    pub fee_ccy: Option<String>,
    #[serde(rename="rebate", skip_serializing_if = "Option::is_none")]
    pub rebate: Option<String>,
    #[serde(rename="rebateCcy", skip_serializing_if = "Option::is_none")]
    pub rebate_ccy: Option<String>,
    #[serde(rename="avgPx", skip_serializing_if = "Option::is_none")]
    pub avg_px: Option<String>,
    #[serde(rename="accFillSz", skip_serializing_if = "Option::is_none")]
    pub acc_fill_sz: Option<String>,
    #[serde(rename="posSide", skip_serializing_if = "Option::is_none")]
    pub pos_side: Option<String>,
    #[serde(rename="pnl", skip_serializing_if = "Option::is_none")]
    pub pnl: Option<String>,
    #[serde(rename="ctVal", skip_serializing_if = "Option::is_none")]
    pub ct_val: Option<String>,
    #[serde(rename="lever", skip_serializing_if = "Option::is_none")]
    pub lever: Option<String>,
    #[serde(rename="pTime", skip_serializing_if = "Option::is_none")]
    pub p_time: Option<String>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

