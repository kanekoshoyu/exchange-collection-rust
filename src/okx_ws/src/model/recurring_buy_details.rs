#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// RecurringBuyDetails represents a RecurringBuyDetails model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct RecurringBuyDetails {
    #[serde(rename="ccy", skip_serializing_if = "Option::is_none")]
    pub ccy: Option<String>,
    #[serde(rename="ratio", skip_serializing_if = "Option::is_none")]
    pub ratio: Option<String>,
    #[serde(rename="totalAmt", skip_serializing_if = "Option::is_none")]
    pub total_amt: Option<String>,
    #[serde(rename="profit", skip_serializing_if = "Option::is_none")]
    pub profit: Option<String>,
    #[serde(rename="avgPx", skip_serializing_if = "Option::is_none")]
    pub avg_px: Option<String>,
    #[serde(rename="px", skip_serializing_if = "Option::is_none")]
    pub px: Option<String>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

