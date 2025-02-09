#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// MarkPricePushResponse represents a MarkPricePushResponse model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct MarkPricePushResponse {
    #[serde(rename="channel", skip_serializing_if = "Option::is_none")]
    pub channel: Option<String>,
    #[serde(rename="instType", skip_serializing_if = "Option::is_none")]
    pub inst_type: Option<Box<InstTypeEnum>>,
    #[serde(rename="data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<MarkPriceData>>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

