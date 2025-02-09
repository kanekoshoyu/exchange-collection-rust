#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// AttachAlgoOrds represents a AttachAlgoOrds model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct AttachAlgoOrds {
    #[serde(rename="attachAlgoClOrdId", skip_serializing_if = "Option::is_none")]
    pub attach_algo_cl_ord_id: Option<String>,
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
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

