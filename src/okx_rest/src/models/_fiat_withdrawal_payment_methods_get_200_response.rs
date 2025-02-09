/*
 * OKX API
 *
 * Welcome to OKX Developer document!   excluded below endpoints as they are if you need them please add and commit to https://github.com/kanekoshoyu/exchange-collection): - Trading Account (this might become needed, will add when we need it) - Block Trading - Financial Producer - Affiliate - Status - Announcement 
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct FiatWithdrawalPaymentMethodsGet200Response {
    /// Response code
    #[serde(rename = "code", skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// Response message
    #[serde(rename = "msg", skip_serializing_if = "Option::is_none")]
    pub msg: Option<String>,
    /// Request received timestamp
    #[serde(rename = "inTime", skip_serializing_if = "Option::is_none")]
    pub in_time: Option<String>,
    /// Response sent timestamp
    #[serde(rename = "outTime", skip_serializing_if = "Option::is_none")]
    pub out_time: Option<String>,
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<models::WithdrawalPaymentMethodsResponse>>,
}

impl FiatWithdrawalPaymentMethodsGet200Response {
    pub fn new() -> FiatWithdrawalPaymentMethodsGet200Response {
        FiatWithdrawalPaymentMethodsGet200Response {
            code: None,
            msg: None,
            in_time: None,
            out_time: None,
            data: None,
        }
    }
}

