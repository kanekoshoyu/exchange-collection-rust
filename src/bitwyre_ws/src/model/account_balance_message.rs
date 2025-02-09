#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// AccountBalanceMessage represents a AccountBalanceMessage model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct AccountBalanceMessage {
    #[serde(rename="assets")]
    pub assets: std::collections::HashMap<String, AssetUnion>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

