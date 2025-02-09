#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// KlineParams represents a KlineParams model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct KlineParams {
    #[serde(rename="realtimeInterval", skip_serializing_if = "Option::is_none")]
    pub realtime_interval: Option<String>,
    #[serde(rename="klineType", skip_serializing_if = "Option::is_none")]
    pub kline_type: Option<String>,
    #[serde(rename="binary", skip_serializing_if = "Option::is_none")]
    pub binary: Option<bool>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

