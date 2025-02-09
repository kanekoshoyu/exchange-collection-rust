#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// AlgoOrderPushData represents a AlgoOrderPushData model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct AlgoOrderPushData {
    #[serde(rename="instType", skip_serializing_if = "Option::is_none")]
    pub inst_type: Option<String>,
    #[serde(rename="instId", skip_serializing_if = "Option::is_none")]
    pub inst_id: Option<String>,
    #[serde(rename="ccy", skip_serializing_if = "Option::is_none")]
    pub ccy: Option<String>,
    #[serde(rename="ordId", skip_serializing_if = "Option::is_none")]
    pub ord_id: Option<String>,
    #[serde(rename="ordIdList", skip_serializing_if = "Option::is_none")]
    pub ord_id_list: Option<Vec<String>>,
    #[serde(rename="algoId", skip_serializing_if = "Option::is_none")]
    pub algo_id: Option<String>,
    #[serde(rename="clOrdId", skip_serializing_if = "Option::is_none")]
    pub cl_ord_id: Option<String>,
    #[serde(rename="sz", skip_serializing_if = "Option::is_none")]
    pub sz: Option<String>,
    #[serde(rename="ordType", skip_serializing_if = "Option::is_none")]
    pub ord_type: Option<Box<OrderEnum>>,
    #[serde(rename="side", skip_serializing_if = "Option::is_none")]
    pub side: Option<String>,
    #[serde(rename="posSide", skip_serializing_if = "Option::is_none")]
    pub pos_side: Option<Box<PosEnum>>,
    #[serde(rename="tdMode", skip_serializing_if = "Option::is_none")]
    pub td_mode: Option<String>,
    #[serde(rename="tgtCcy", skip_serializing_if = "Option::is_none")]
    pub tgt_ccy: Option<String>,
    #[serde(rename="lever", skip_serializing_if = "Option::is_none")]
    pub lever: Option<String>,
    #[serde(rename="state", skip_serializing_if = "Option::is_none")]
    pub state: Option<Box<StateEnum>>,
    #[serde(rename="tpTriggerPx", skip_serializing_if = "Option::is_none")]
    pub tp_trigger_px: Option<String>,
    #[serde(rename="tpTriggerPxType", skip_serializing_if = "Option::is_none")]
    pub tp_trigger_px_type: Option<String>,
    #[serde(rename="tpOrdPx", skip_serializing_if = "Option::is_none")]
    pub tp_ord_px: Option<String>,
    #[serde(rename="slTriggerPx", skip_serializing_if = "Option::is_none")]
    pub sl_trigger_px: Option<String>,
    #[serde(rename="slTriggerPxType", skip_serializing_if = "Option::is_none")]
    pub sl_trigger_px_type: Option<String>,
    #[serde(rename="slOrdPx", skip_serializing_if = "Option::is_none")]
    pub sl_ord_px: Option<String>,
    #[serde(rename="triggerPx", skip_serializing_if = "Option::is_none")]
    pub trigger_px: Option<String>,
    #[serde(rename="triggerPxType", skip_serializing_if = "Option::is_none")]
    pub trigger_px_type: Option<String>,
    #[serde(rename="ordPx", skip_serializing_if = "Option::is_none")]
    pub ord_px: Option<String>,
    #[serde(rename="last", skip_serializing_if = "Option::is_none")]
    pub last: Option<String>,
    #[serde(rename="actualSz", skip_serializing_if = "Option::is_none")]
    pub actual_sz: Option<String>,
    #[serde(rename="actualPx", skip_serializing_if = "Option::is_none")]
    pub actual_px: Option<String>,
    #[serde(rename="notionalUsd", skip_serializing_if = "Option::is_none")]
    pub notional_usd: Option<String>,
    #[serde(rename="tag", skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    #[serde(rename="actualSide", skip_serializing_if = "Option::is_none")]
    pub actual_side: Option<String>,
    #[serde(rename="triggerTime", skip_serializing_if = "Option::is_none")]
    pub trigger_time: Option<String>,
    #[serde(rename="reduceOnly", skip_serializing_if = "Option::is_none")]
    pub reduce_only: Option<bool>,
    #[serde(rename="failCode", skip_serializing_if = "Option::is_none")]
    pub fail_code: Option<String>,
    #[serde(rename="algoClOrdId", skip_serializing_if = "Option::is_none")]
    pub algo_cl_ord_id: Option<String>,
    #[serde(rename="reqId", skip_serializing_if = "Option::is_none")]
    pub req_id: Option<String>,
    #[serde(rename="amendResult", skip_serializing_if = "Option::is_none")]
    pub amend_result: Option<String>,
    #[serde(rename="amendPxOnTriggerType", skip_serializing_if = "Option::is_none")]
    pub amend_px_on_trigger_type: Option<String>,
    #[serde(rename="attachAlgoOrds", skip_serializing_if = "Option::is_none")]
    pub attach_algo_ords: Option<Vec<AttachAlgoOrds>>,
    #[serde(rename="linkedOrd", skip_serializing_if = "Option::is_none")]
    pub linked_ord: Option<Box<LinkData>>,
    #[serde(rename="cTime", skip_serializing_if = "Option::is_none")]
    pub c_time: Option<String>,
    #[serde(rename="uTime", skip_serializing_if = "Option::is_none")]
    pub u_time: Option<String>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

