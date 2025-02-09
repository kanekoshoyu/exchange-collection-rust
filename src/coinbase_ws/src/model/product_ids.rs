#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// ProductIds represents a ProductIds model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct ProductIds {
    #[serde(rename="name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename="product_ids", skip_serializing_if = "Option::is_none")]
    pub product_ids: Option<Vec<String>>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

