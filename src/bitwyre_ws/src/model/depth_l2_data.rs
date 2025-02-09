#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// DepthL2Data represents a DepthL2Data model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct DepthL2Data {
    #[serde(rename="instrument", skip_serializing_if = "Option::is_none")]
    pub instrument: Option<String>,
    #[serde(rename="bids", skip_serializing_if = "Option::is_none")]
    pub bids: Option<Vec<DepthL2Order>>,
    #[serde(rename="asks", skip_serializing_if = "Option::is_none")]
    pub asks: Option<Vec<DepthL2Order>>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

