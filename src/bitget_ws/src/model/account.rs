#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// Account represents a Account model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Account {
    #[serde(rename="coin")]
    pub coin: String,
    #[serde(rename="available")]
    pub available: String,
    #[serde(rename="frozen")]
    pub frozen: String,
    #[serde(rename="locked")]
    pub locked: String,
    #[serde(rename="limitAvailable")]
    pub limit_available: String,
    #[serde(rename="uTime")]
    pub u_time: String,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

