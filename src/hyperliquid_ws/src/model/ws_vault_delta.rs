#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// WsVaultDelta represents a WsVaultDelta model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct WsVaultDelta {
    #[serde(rename="type")]
    pub reserved_type: Box<VaultEnum>,
    #[serde(rename="vault")]
    pub vault: String,
    #[serde(rename="usdc")]
    pub usdc: f64,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

