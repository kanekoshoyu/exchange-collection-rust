#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// WsVaultLeaderCommission represents a WsVaultLeaderCommission model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct WsVaultLeaderCommission {
    #[serde(rename="type")]
    pub reserved_type: Box<CommissionEnum>,
    #[serde(rename="user")]
    pub user: String,
    #[serde(rename="usdc")]
    pub usdc: f64,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

