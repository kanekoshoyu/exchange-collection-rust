#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// RebateData represents a RebateData model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct RebateData {
    #[serde(rename="rebate", skip_serializing_if = "Option::is_none")]
    pub rebate: Option<String>,
    #[serde(rename="rebateCcy", skip_serializing_if = "Option::is_none")]
    pub rebate_ccy: Option<String>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

