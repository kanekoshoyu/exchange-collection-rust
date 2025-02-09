#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// CancelOrderResponse represents a CancelOrderResponse model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct CancelOrderResponse {
    #[serde(rename="id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename="op", skip_serializing_if = "Option::is_none")]
    pub op: Option<String>,
    #[serde(rename="code", skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(rename="msg", skip_serializing_if = "Option::is_none")]
    pub msg: Option<String>,
    #[serde(rename="data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<CancelOrderResponseData>>,
    #[serde(rename="inTime", skip_serializing_if = "Option::is_none")]
    pub in_time: Option<String>,
    #[serde(rename="outTime", skip_serializing_if = "Option::is_none")]
    pub out_time: Option<String>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

