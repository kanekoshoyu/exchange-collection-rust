#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// ContractData represents a ContractData model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct ContractData {
    #[serde(rename="algoId", skip_serializing_if = "Option::is_none")]
    pub algo_id: Option<String>,
    #[serde(rename="algoClOrdId", skip_serializing_if = "Option::is_none")]
    pub algo_cl_ord_id: Option<String>,
    #[serde(rename="instType", skip_serializing_if = "Option::is_none")]
    pub inst_type: Option<String>,
    #[serde(rename="instId", skip_serializing_if = "Option::is_none")]
    pub inst_id: Option<String>,
    #[serde(rename="cTime", skip_serializing_if = "Option::is_none")]
    pub c_time: Option<String>,
    #[serde(rename="uTime", skip_serializing_if = "Option::is_none")]
    pub u_time: Option<String>,
    #[serde(rename="algoOrdType", skip_serializing_if = "Option::is_none")]
    pub algo_ord_type: Option<String>,
    #[serde(rename="contract_grid", skip_serializing_if = "Option::is_none")]
    pub contract_grid: Option<Box<ContractGrid>>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

