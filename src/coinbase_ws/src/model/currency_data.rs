#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// CurrencyData represents a CurrencyData model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct CurrencyData {
    #[serde(rename="id")]
    pub id: String,
    #[serde(rename="name")]
    pub name: String,
    #[serde(rename="display_name")]
    pub display_name: String,
    #[serde(rename="min_size")]
    pub min_size: String,
    #[serde(rename="status")]
    pub status: String,
    #[serde(rename="status_message", skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
    #[serde(rename="max_precision")]
    pub max_precision: String,
    #[serde(rename="convertible_to")]
    pub convertible_to: Vec<String>,
    #[serde(rename="details")]
    pub details: std::collections::HashMap<String, serde_json::Value>,
    #[serde(rename="default_network")]
    pub default_network: String,
    #[serde(rename="supported_networks")]
    pub supported_networks: Vec<NetworkData>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

