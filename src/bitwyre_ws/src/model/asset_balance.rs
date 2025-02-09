#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// AssetBalance represents a AssetBalance model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct AssetBalance {
    #[serde(rename="locked_balance", skip_serializing_if = "Option::is_none")]
    pub locked_balance: Option<String>,
    #[serde(rename="available_balance", skip_serializing_if = "Option::is_none")]
    pub available_balance: Option<String>,
    #[serde(rename="total_balance", skip_serializing_if = "Option::is_none")]
    pub total_balance: Option<String>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

