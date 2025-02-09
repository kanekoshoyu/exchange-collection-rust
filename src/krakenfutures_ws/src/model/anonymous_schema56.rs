#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// AnonymousSchema56 represents a AnonymousSchema56 model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct AnonymousSchema56 {
    #[serde(rename="quantity", skip_serializing_if = "Option::is_none")]
    pub quantity: Option<f64>,
    #[serde(rename="value", skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
    #[serde(rename="collateral_value", skip_serializing_if = "Option::is_none")]
    pub collateral_value: Option<f64>,
    #[serde(rename="available", skip_serializing_if = "Option::is_none")]
    pub available: Option<f64>,
    #[serde(rename="haircut", skip_serializing_if = "Option::is_none")]
    pub haircut: Option<f64>,
    #[serde(rename="conversion_spread", skip_serializing_if = "Option::is_none")]
    pub conversion_spread: Option<f64>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

