#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// ContractGrid represents a ContractGrid model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct ContractGrid {
    #[serde(rename="state", skip_serializing_if = "Option::is_none")]
    pub state: Option<Box<StateEnum>>,
    #[serde(rename="rebateTrans", skip_serializing_if = "Option::is_none")]
    pub rebate_trans: Option<Vec<RebateData>>,
    #[serde(rename="triggerParams", skip_serializing_if = "Option::is_none")]
    pub trigger_params: Option<Vec<TriggerData>>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

