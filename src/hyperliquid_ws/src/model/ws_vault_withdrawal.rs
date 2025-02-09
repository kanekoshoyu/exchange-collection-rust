#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// WsVaultWithdrawal represents a WsVaultWithdrawal model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct WsVaultWithdrawal {
    #[serde(rename="type")]
    pub reserved_type: Box<VaultWithdrawEnum>,
    #[serde(rename="vault")]
    pub vault: String,
    #[serde(rename="user")]
    pub user: String,
    #[serde(rename="requestedUsd")]
    pub requested_usd: f64,
    #[serde(rename="commission")]
    pub commission: f64,
    #[serde(rename="closingCost")]
    pub closing_cost: f64,
    #[serde(rename="basis")]
    pub basis: f64,
    #[serde(rename="netWithdrawnUsd")]
    pub net_withdrawn_usd: f64,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

