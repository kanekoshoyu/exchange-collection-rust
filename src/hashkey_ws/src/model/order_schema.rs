#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// OrderSchema represents a OrderSchema model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct OrderSchema {
    #[serde(rename="e", skip_serializing_if = "Option::is_none")]
    pub e: Option<String>,
    #[serde(rename="E", skip_serializing_if = "Option::is_none")]
    pub reserved_e: Option<i32>,
    #[serde(rename="s", skip_serializing_if = "Option::is_none")]
    pub s: Option<String>,
    #[serde(rename="c", skip_serializing_if = "Option::is_none")]
    pub c: Option<String>,
    #[serde(rename="S", skip_serializing_if = "Option::is_none")]
    pub reserved_s: Option<String>,
    #[serde(rename="o", skip_serializing_if = "Option::is_none")]
    pub o: Option<String>,
    #[serde(rename="f", skip_serializing_if = "Option::is_none")]
    pub f: Option<String>,
    #[serde(rename="q", skip_serializing_if = "Option::is_none")]
    pub q: Option<String>,
    #[serde(rename="p", skip_serializing_if = "Option::is_none")]
    pub p: Option<String>,
    #[serde(rename="reqAmt", skip_serializing_if = "Option::is_none")]
    pub req_amt: Option<String>,
    #[serde(rename="x", skip_serializing_if = "Option::is_none")]
    pub x: Option<String>,
    #[serde(rename="d", skip_serializing_if = "Option::is_none")]
    pub d: Option<String>,
    #[serde(rename="i", skip_serializing_if = "Option::is_none")]
    pub i: Option<i32>,
    #[serde(rename="l", skip_serializing_if = "Option::is_none")]
    pub l: Option<String>,
    #[serde(rename="r", skip_serializing_if = "Option::is_none")]
    pub r: Option<String>,
    #[serde(rename="z", skip_serializing_if = "Option::is_none")]
    pub z: Option<String>,
    #[serde(rename="L", skip_serializing_if = "Option::is_none")]
    pub reserved_l: Option<String>,
    #[serde(rename="V", skip_serializing_if = "Option::is_none")]
    pub v: Option<String>,
    #[serde(rename="n", skip_serializing_if = "Option::is_none")]
    pub n: Option<String>,
    #[serde(rename="N", skip_serializing_if = "Option::is_none")]
    pub reserved_n: Option<String>,
    #[serde(rename="u", skip_serializing_if = "Option::is_none")]
    pub u: Option<bool>,
    #[serde(rename="w", skip_serializing_if = "Option::is_none")]
    pub w: Option<bool>,
    #[serde(rename="m", skip_serializing_if = "Option::is_none")]
    pub m: Option<bool>,
    #[serde(rename="O", skip_serializing_if = "Option::is_none")]
    pub reserved_o: Option<i32>,
    #[serde(rename="Z", skip_serializing_if = "Option::is_none")]
    pub reserved_z: Option<String>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

