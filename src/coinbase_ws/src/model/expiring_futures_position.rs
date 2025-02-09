#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// ExpiringFuturesPosition represents a ExpiringFuturesPosition model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct ExpiringFuturesPosition {
    #[serde(rename="product_id", skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    #[serde(rename="side", skip_serializing_if = "Option::is_none")]
    pub side: Option<String>,
    #[serde(rename="number_of_contracts", skip_serializing_if = "Option::is_none")]
    pub number_of_contracts: Option<String>,
    #[serde(rename="realized_pnl", skip_serializing_if = "Option::is_none")]
    pub realized_pnl: Option<String>,
    #[serde(rename="unrealized_pnl", skip_serializing_if = "Option::is_none")]
    pub unrealized_pnl: Option<String>,
    #[serde(rename="entry_price", skip_serializing_if = "Option::is_none")]
    pub entry_price: Option<String>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

