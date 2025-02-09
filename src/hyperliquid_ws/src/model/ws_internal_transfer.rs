#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// WsInternalTransfer represents a WsInternalTransfer model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct WsInternalTransfer {
    #[serde(rename="type")]
    pub reserved_type: Box<TransferEnum>,
    #[serde(rename="usdc")]
    pub usdc: f64,
    #[serde(rename="user")]
    pub user: String,
    #[serde(rename="destination")]
    pub destination: String,
    #[serde(rename="fee")]
    pub fee: f64,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

