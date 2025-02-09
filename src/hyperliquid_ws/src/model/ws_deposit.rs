#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// WsDeposit represents a WsDeposit model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct WsDeposit {
    #[serde(rename="type")]
    pub reserved_type: Box<WsDepositEnum>,
    #[serde(rename="usdc")]
    pub usdc: f64,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

