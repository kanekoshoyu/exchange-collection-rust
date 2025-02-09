#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// BalanceRequestArgs represents a BalanceRequestArgs model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct BalanceRequestArgs {
    #[serde(rename="channel", skip_serializing_if = "Option::is_none")]
    pub channel: Option<Box<BalanceEnum>>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

