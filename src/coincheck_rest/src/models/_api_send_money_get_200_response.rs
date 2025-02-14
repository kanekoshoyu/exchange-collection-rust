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
pub struct ApiSendMoneyGet200Response {
    /// Indicates success of the API call.
    #[serde(rename = "success", skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
    /// List of sending transactions.
    #[serde(rename = "sends", skip_serializing_if = "Option::is_none")]
    pub sends: Option<Vec<models::MoneyTransferResponse>>,
}

impl ApiSendMoneyGet200Response {
    pub fn new() -> ApiSendMoneyGet200Response {
        ApiSendMoneyGet200Response {
            success: None,
            sends: None,
        }
    }
}

