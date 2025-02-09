#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// TicketSchema represents a TicketSchema model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct TicketSchema {
    #[serde(rename="e", skip_serializing_if = "Option::is_none")]
    pub e: Option<String>,
    #[serde(rename="E", skip_serializing_if = "Option::is_none")]
    pub reserved_e: Option<String>,
    #[serde(rename="s", skip_serializing_if = "Option::is_none")]
    pub s: Option<String>,
    #[serde(rename="q", skip_serializing_if = "Option::is_none")]
    pub q: Option<String>,
    #[serde(rename="t", skip_serializing_if = "Option::is_none")]
    pub t: Option<String>,
    #[serde(rename="p", skip_serializing_if = "Option::is_none")]
    pub p: Option<String>,
    #[serde(rename="T", skip_serializing_if = "Option::is_none")]
    pub reserved_t: Option<String>,
    #[serde(rename="o", skip_serializing_if = "Option::is_none")]
    pub o: Option<String>,
    #[serde(rename="c", skip_serializing_if = "Option::is_none")]
    pub c: Option<String>,
    #[serde(rename="O", skip_serializing_if = "Option::is_none")]
    pub reserved_o: Option<String>,
    #[serde(rename="a", skip_serializing_if = "Option::is_none")]
    pub a: Option<String>,
    #[serde(rename="A", skip_serializing_if = "Option::is_none")]
    pub reserved_a: Option<i32>,
    #[serde(rename="m", skip_serializing_if = "Option::is_none")]
    pub m: Option<bool>,
    #[serde(rename="S", skip_serializing_if = "Option::is_none")]
    pub reserved_s: Option<String>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

