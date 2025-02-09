#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// OrderRequest represents a OrderRequest model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct OrderRequest {
    #[serde(rename="instrument", skip_serializing_if = "Option::is_none")]
    pub instrument: Option<String>,
    #[serde(rename="ordtype", skip_serializing_if = "Option::is_none")]
    pub ordtype: Option<i32>,
    #[serde(rename="side", skip_serializing_if = "Option::is_none")]
    pub side: Option<i32>,
    #[serde(rename="orderqty", skip_serializing_if = "Option::is_none")]
    pub orderqty: Option<String>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

