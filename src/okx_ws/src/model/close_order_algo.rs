#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// CloseOrderAlgo represents a CloseOrderAlgo model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct CloseOrderAlgo {
    #[serde(rename="algoId", skip_serializing_if = "Option::is_none")]
    pub algo_id: Option<String>,
    #[serde(rename="slTriggerPx", skip_serializing_if = "Option::is_none")]
    pub sl_trigger_px: Option<String>,
    #[serde(rename="tpTriggerPx", skip_serializing_if = "Option::is_none")]
    pub tp_trigger_px: Option<String>,
    #[serde(rename="closeFraction", skip_serializing_if = "Option::is_none")]
    pub close_fraction: Option<String>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

