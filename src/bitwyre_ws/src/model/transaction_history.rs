#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// TransactionHistory represents a TransactionHistory model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct TransactionHistory {
    #[serde(rename="id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename="transaction_uuid", skip_serializing_if = "Option::is_none")]
    pub transaction_uuid: Option<String>,
    #[serde(rename="tx_id", skip_serializing_if = "Option::is_none")]
    pub tx_id: Option<String>,
    #[serde(rename="user_uuid", skip_serializing_if = "Option::is_none")]
    pub user_uuid: Option<String>,
    #[serde(rename="transaction_type", skip_serializing_if = "Option::is_none")]
    pub transaction_type: Option<i32>,
    #[serde(rename="asset", skip_serializing_if = "Option::is_none")]
    pub asset: Option<String>,
    #[serde(rename="status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename="type", skip_serializing_if = "Option::is_none")]
    pub reserved_type: Option<String>,
    #[serde(rename="provider", skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[serde(rename="address", skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(rename="gross_amount", skip_serializing_if = "Option::is_none")]
    pub gross_amount: Option<String>,
    #[serde(rename="fee", skip_serializing_if = "Option::is_none")]
    pub fee: Option<String>,
    #[serde(rename="nett_amount", skip_serializing_if = "Option::is_none")]
    pub nett_amount: Option<String>,
    #[serde(rename="network_confirmation", skip_serializing_if = "Option::is_none")]
    pub network_confirmation: Option<i32>,
    #[serde(rename="centralized_confirmation", skip_serializing_if = "Option::is_none")]
    pub centralized_confirmation: Option<i32>,
    #[serde(rename="submit_time", skip_serializing_if = "Option::is_none")]
    pub submit_time: Option<i32>,
    #[serde(rename="success_time", skip_serializing_if = "Option::is_none")]
    pub success_time: Option<i32>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

