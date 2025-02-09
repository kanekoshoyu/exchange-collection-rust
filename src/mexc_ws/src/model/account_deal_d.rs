#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// AccountDealD represents a AccountDealD model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct AccountDealD {
    #[serde(rename="p", skip_serializing_if = "Option::is_none")]
    pub p: Option<String>,
    #[serde(rename="v", skip_serializing_if = "Option::is_none")]
    pub v: Option<String>,
    #[serde(rename="a", skip_serializing_if = "Option::is_none")]
    pub a: Option<String>,
    #[serde(rename="S", skip_serializing_if = "Option::is_none")]
    pub s: Option<i32>,
    #[serde(rename="t", skip_serializing_if = "Option::is_none")]
    pub t: Option<String>,
    #[serde(rename="c", skip_serializing_if = "Option::is_none")]
    pub c: Option<String>,
    #[serde(rename="i", skip_serializing_if = "Option::is_none")]
    pub i: Option<String>,
    #[serde(rename="m", skip_serializing_if = "Option::is_none")]
    pub m: Option<i32>,
    #[serde(rename="st", skip_serializing_if = "Option::is_none")]
    pub st: Option<i32>,
    #[serde(rename="n", skip_serializing_if = "Option::is_none")]
    pub n: Option<String>,
    #[serde(rename="N", skip_serializing_if = "Option::is_none")]
    pub reserved_n: Option<String>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

