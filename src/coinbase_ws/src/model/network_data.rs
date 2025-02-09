#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// NetworkData represents a NetworkData model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct NetworkData {
    #[serde(rename="id")]
    pub id: String,
    #[serde(rename="name")]
    pub name: String,
    #[serde(rename="status")]
    pub status: String,
    #[serde(rename="contract_address", skip_serializing_if = "Option::is_none")]
    pub contract_address: Option<String>,
    #[serde(rename="crypto_address_link")]
    pub crypto_address_link: String,
    #[serde(rename="crypto_transaction_link")]
    pub crypto_transaction_link: String,
    #[serde(rename="min_withdrawal_amount")]
    pub min_withdrawal_amount: f64,
    #[serde(rename="max_withdrawal_amount")]
    pub max_withdrawal_amount: f64,
    #[serde(rename="network_confirmations")]
    pub network_confirmations: i32,
    #[serde(rename="processing_time_seconds")]
    pub processing_time_seconds: i32,
    #[serde(rename="destination_tag_regex", skip_serializing_if = "Option::is_none")]
    pub destination_tag_regex: Option<String>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

