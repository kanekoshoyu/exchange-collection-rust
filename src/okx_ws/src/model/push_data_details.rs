#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// PushDataDetails represents a PushDataDetails model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct PushDataDetails {
    #[serde(rename="uTime", skip_serializing_if = "Option::is_none")]
    pub u_time: Option<String>,
    #[serde(rename="totalEq", skip_serializing_if = "Option::is_none")]
    pub total_eq: Option<String>,
    #[serde(rename="isoEq", skip_serializing_if = "Option::is_none")]
    pub iso_eq: Option<String>,
    #[serde(rename="adjEq", skip_serializing_if = "Option::is_none")]
    pub adj_eq: Option<String>,
    #[serde(rename="ordFroz", skip_serializing_if = "Option::is_none")]
    pub ord_froz: Option<String>,
    #[serde(rename="imr", skip_serializing_if = "Option::is_none")]
    pub imr: Option<String>,
    #[serde(rename="mmr", skip_serializing_if = "Option::is_none")]
    pub mmr: Option<String>,
    #[serde(rename="borrowFroz", skip_serializing_if = "Option::is_none")]
    pub borrow_froz: Option<String>,
    #[serde(rename="mgnRatio", skip_serializing_if = "Option::is_none")]
    pub mgn_ratio: Option<String>,
    #[serde(rename="notionalUsd", skip_serializing_if = "Option::is_none")]
    pub notional_usd: Option<String>,
    #[serde(rename="upl", skip_serializing_if = "Option::is_none")]
    pub upl: Option<String>,
    #[serde(rename="details", skip_serializing_if = "Option::is_none")]
    pub details: Option<Vec<PushDetails>>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

