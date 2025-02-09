#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// LiquidationDetails represents a LiquidationDetails model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct LiquidationDetails {
    #[serde(rename="side", skip_serializing_if = "Option::is_none")]
    pub side: Option<String>,
    #[serde(rename="posSide", skip_serializing_if = "Option::is_none")]
    pub pos_side: Option<String>,
    #[serde(rename="bkPx", skip_serializing_if = "Option::is_none")]
    pub bk_px: Option<String>,
    #[serde(rename="sz", skip_serializing_if = "Option::is_none")]
    pub sz: Option<String>,
    #[serde(rename="bkLoss", skip_serializing_if = "Option::is_none")]
    pub bk_loss: Option<String>,
    #[serde(rename="ccy", skip_serializing_if = "Option::is_none")]
    pub ccy: Option<String>,
    #[serde(rename="ts", skip_serializing_if = "Option::is_none")]
    pub ts: Option<String>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

