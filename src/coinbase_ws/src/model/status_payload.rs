#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// StatusPayload represents a StatusPayload model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct StatusPayload {
    #[serde(rename="type")]
    pub reserved_type: String,
    #[serde(rename="products")]
    pub products: Vec<ProductData>,
    #[serde(rename="currencies")]
    pub currencies: Vec<CurrencyData>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

