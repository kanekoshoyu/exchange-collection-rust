#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// LiquidationData represents a LiquidationData model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct LiquidationData {
    #[serde(rename="instType", skip_serializing_if = "Option::is_none")]
    pub inst_type: Option<String>,
    #[serde(rename="instId", skip_serializing_if = "Option::is_none")]
    pub inst_id: Option<String>,
    #[serde(rename="uly", skip_serializing_if = "Option::is_none")]
    pub uly: Option<String>,
    #[serde(rename="details", skip_serializing_if = "Option::is_none")]
    pub details: Option<Vec<LiquidationDetails>>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

