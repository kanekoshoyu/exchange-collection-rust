#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// FuturesBalanceSummaryEvent represents a FuturesBalanceSummaryEvent model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct FuturesBalanceSummaryEvent {
    #[serde(rename="type")]
    pub reserved_type: String,
    #[serde(rename="fcm_balance_summary")]
    pub fcm_balance_summary: Box<FcmBalanceSummary>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

