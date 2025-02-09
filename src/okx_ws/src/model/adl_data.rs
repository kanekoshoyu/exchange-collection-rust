#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// AdlData represents a AdlData model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct AdlData {
    #[serde(rename="instType", skip_serializing_if = "Option::is_none")]
    pub inst_type: Option<String>,
    #[serde(rename="instFamily", skip_serializing_if = "Option::is_none")]
    pub inst_family: Option<String>,
    #[serde(rename="state", skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename="bal", skip_serializing_if = "Option::is_none")]
    pub bal: Option<String>,
    #[serde(rename="ccy", skip_serializing_if = "Option::is_none")]
    pub ccy: Option<String>,
    #[serde(rename="maxBal", skip_serializing_if = "Option::is_none")]
    pub max_bal: Option<String>,
    #[serde(rename="maxBalTs", skip_serializing_if = "Option::is_none")]
    pub max_bal_ts: Option<String>,
    #[serde(rename="decRate", skip_serializing_if = "Option::is_none")]
    pub dec_rate: Option<String>,
    #[serde(rename="adlType", skip_serializing_if = "Option::is_none")]
    pub adl_type: Option<String>,
    #[serde(rename="adlBal", skip_serializing_if = "Option::is_none")]
    pub adl_bal: Option<String>,
    #[serde(rename="adlRate", skip_serializing_if = "Option::is_none")]
    pub adl_rate: Option<String>,
    #[serde(rename="adlRecBal", skip_serializing_if = "Option::is_none")]
    pub adl_rec_bal: Option<String>,
    #[serde(rename="adlRecRate", skip_serializing_if = "Option::is_none")]
    pub adl_rec_rate: Option<String>,
    #[serde(rename="ts", skip_serializing_if = "Option::is_none")]
    pub ts: Option<String>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

