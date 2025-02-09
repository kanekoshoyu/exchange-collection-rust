#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// BalanceData represents a BalanceData model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct BalanceData {
    #[serde(rename="ccy", skip_serializing_if = "Option::is_none")]
    pub ccy: Option<String>,
    #[serde(rename="cashBal", skip_serializing_if = "Option::is_none")]
    pub cash_bal: Option<String>,
    #[serde(rename="uTime", skip_serializing_if = "Option::is_none")]
    pub u_time: Option<String>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

