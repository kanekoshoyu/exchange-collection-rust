#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// IndividualSymbolRollingWindowStatistics represents a IndividualSymbolRollingWindowStatistics model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct IndividualSymbolRollingWindowStatistics {
    #[serde(rename="e", skip_serializing_if = "Option::is_none")]
    pub e: Option<String>,
    #[serde(rename="E", skip_serializing_if = "Option::is_none")]
    pub reserved_e: Option<i64>,
    #[serde(rename="s", skip_serializing_if = "Option::is_none")]
    pub s: Option<String>,
    #[serde(rename="p", skip_serializing_if = "Option::is_none")]
    pub p: Option<String>,
    #[serde(rename="P", skip_serializing_if = "Option::is_none")]
    pub reserved_p: Option<String>,
    #[serde(rename="o", skip_serializing_if = "Option::is_none")]
    pub o: Option<String>,
    #[serde(rename="h", skip_serializing_if = "Option::is_none")]
    pub h: Option<String>,
    #[serde(rename="l", skip_serializing_if = "Option::is_none")]
    pub l: Option<String>,
    #[serde(rename="c", skip_serializing_if = "Option::is_none")]
    pub c: Option<String>,
    #[serde(rename="w", skip_serializing_if = "Option::is_none")]
    pub w: Option<String>,
    #[serde(rename="v", skip_serializing_if = "Option::is_none")]
    pub v: Option<String>,
    #[serde(rename="q", skip_serializing_if = "Option::is_none")]
    pub q: Option<String>,
    #[serde(rename="O", skip_serializing_if = "Option::is_none")]
    pub reserved_o: Option<i32>,
    #[serde(rename="C", skip_serializing_if = "Option::is_none")]
    pub reserved_c: Option<i64>,
    #[serde(rename="F", skip_serializing_if = "Option::is_none")]
    pub f: Option<i32>,
    #[serde(rename="L", skip_serializing_if = "Option::is_none")]
    pub reserved_l: Option<i32>,
    #[serde(rename="n", skip_serializing_if = "Option::is_none")]
    pub n: Option<i32>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

