/*
 * Coincheck Crypto Exchange
 *
 * Welcome to Coincheck API document! 
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApiDepositMoneyGet200Response {
    /// Indicates success of the API call.
    #[serde(rename = "success", skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
    /// List of deposit transactions.
    #[serde(rename = "deposits", skip_serializing_if = "Option::is_none")]
    pub deposits: Option<Vec<models::MoneyDepositResponse>>,
}

impl ApiDepositMoneyGet200Response {
    pub fn new() -> ApiDepositMoneyGet200Response {
        ApiDepositMoneyGet200Response {
            success: None,
            deposits: None,
        }
    }
}

