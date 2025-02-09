#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// GridItems represents a GridItems model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct GridItems {
    #[serde(rename="algoId", skip_serializing_if = "Option::is_none")]
    pub algo_id: Option<String>,
    #[serde(rename="algoClOrdId", skip_serializing_if = "Option::is_none")]
    pub algo_cl_ord_id: Option<String>,
    #[serde(rename="instType", skip_serializing_if = "Option::is_none")]
    pub inst_type: Option<String>,
    #[serde(rename="instId", skip_serializing_if = "Option::is_none")]
    pub inst_id: Option<String>,
    #[serde(rename="cTime", skip_serializing_if = "Option::is_none")]
    pub c_time: Option<String>,
    #[serde(rename="uTime", skip_serializing_if = "Option::is_none")]
    pub u_time: Option<String>,
    #[serde(rename="avgPx", skip_serializing_if = "Option::is_none")]
    pub avg_px: Option<String>,
    #[serde(rename="ccy", skip_serializing_if = "Option::is_none")]
    pub ccy: Option<String>,
    #[serde(rename="lever", skip_serializing_if = "Option::is_none")]
    pub lever: Option<String>,
    #[serde(rename="liqPx", skip_serializing_if = "Option::is_none")]
    pub liq_px: Option<String>,
    #[serde(rename="posSide", skip_serializing_if = "Option::is_none")]
    pub pos_side: Option<String>,
    #[serde(rename="pos", skip_serializing_if = "Option::is_none")]
    pub pos: Option<String>,
    #[serde(rename="mgnMode", skip_serializing_if = "Option::is_none")]
    pub mgn_mode: Option<String>,
    #[serde(rename="mgnRatio", skip_serializing_if = "Option::is_none")]
    pub mgn_ratio: Option<String>,
    #[serde(rename="imr", skip_serializing_if = "Option::is_none")]
    pub imr: Option<String>,
    #[serde(rename="mmr", skip_serializing_if = "Option::is_none")]
    pub mmr: Option<String>,
    #[serde(rename="upl", skip_serializing_if = "Option::is_none")]
    pub upl: Option<String>,
    #[serde(rename="uplRatio", skip_serializing_if = "Option::is_none")]
    pub upl_ratio: Option<String>,
    #[serde(rename="last", skip_serializing_if = "Option::is_none")]
    pub last: Option<String>,
    #[serde(rename="notionalUsd", skip_serializing_if = "Option::is_none")]
    pub notional_usd: Option<String>,
    #[serde(rename="adl", skip_serializing_if = "Option::is_none")]
    pub adl: Option<String>,
    #[serde(rename="markPx", skip_serializing_if = "Option::is_none")]
    pub mark_px: Option<String>,
    #[serde(rename="pTime", skip_serializing_if = "Option::is_none")]
    pub p_time: Option<String>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

