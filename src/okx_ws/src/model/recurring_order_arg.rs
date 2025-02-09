#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// RecurringOrderArg represents a RecurringOrderArg model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct RecurringOrderArg {
    #[serde(rename="channel", skip_serializing_if = "Option::is_none")]
    pub channel: Option<String>,
    #[serde(rename="instType", skip_serializing_if = "Option::is_none")]
    pub inst_type: Option<String>,
    #[serde(rename="algoId", skip_serializing_if = "Option::is_none")]
    pub algo_id: Option<String>,
    #[serde(rename="uid", skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

