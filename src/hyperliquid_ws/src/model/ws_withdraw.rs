#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// WsWithdraw represents a WsWithdraw model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct WsWithdraw {
    #[serde(rename="type")]
    pub reserved_type: Box<WithdrawEnum>,
    #[serde(rename="usdc")]
    pub usdc: f64,
    #[serde(rename="nonce")]
    pub nonce: f64,
    #[serde(rename="fee")]
    pub fee: f64,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

