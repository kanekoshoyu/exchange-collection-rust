#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// LimitMarketOrderInfo represents a LimitMarketOrderInfo model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct LimitMarketOrderInfo {
    #[serde(rename="a")]
    pub a: String,
    #[serde(rename="o")]
    pub o: i32,
    #[serde(rename="s")]
    pub s: i32,
    #[serde(rename="v")]
    pub v: String,
    #[serde(rename="c", skip_serializing_if = "Option::is_none")]
    pub c: Option<String>,
    #[serde(rename="i")]
    pub i: String,
    #[serde(rename="m")]
    pub m: i32,
    #[serde(rename="p")]
    pub p: String,
    #[serde(rename="ap", skip_serializing_if = "Option::is_none")]
    pub ap: Option<String>,
    #[serde(rename="cv", skip_serializing_if = "Option::is_none")]
    pub cv: Option<String>,
    #[serde(rename="ca", skip_serializing_if = "Option::is_none")]
    pub ca: Option<String>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

