/*
 * Bitget API
 *
 * Welcome to Bitget Developer document! 
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct OrderFillsResponseFillListInnerFeeDetailInner {
    /// Whether fee deduction was applied, e.g., 'yes' or 'no'.
    #[serde(rename = "deduction", skip_serializing_if = "Option::is_none")]
    pub deduction: Option<String>,
    /// The coin in which the fee was charged.
    #[serde(rename = "feeCoin", skip_serializing_if = "Option::is_none")]
    pub fee_coin: Option<String>,
    /// Total fee deducted.
    #[serde(rename = "totalDeductionFee", skip_serializing_if = "Option::is_none")]
    pub total_deduction_fee: Option<String>,
    /// Total fee amount.
    #[serde(rename = "totalFee", skip_serializing_if = "Option::is_none")]
    pub total_fee: Option<String>,
}

impl OrderFillsResponseFillListInnerFeeDetailInner {
    pub fn new() -> OrderFillsResponseFillListInnerFeeDetailInner {
        OrderFillsResponseFillListInnerFeeDetailInner {
            deduction: None,
            fee_coin: None,
            total_deduction_fee: None,
            total_fee: None,
        }
    }
}

