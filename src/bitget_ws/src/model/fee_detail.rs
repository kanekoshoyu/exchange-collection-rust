#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// FeeDetail represents a FeeDetail model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct FeeDetail {
    #[serde(rename="feeCoin")]
    pub fee_coin: String,
    #[serde(rename="deduction")]
    pub deduction: String,
    #[serde(rename="totalDeductionFee")]
    pub total_deduction_fee: String,
    #[serde(rename="totalFee")]
    pub total_fee: String,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

