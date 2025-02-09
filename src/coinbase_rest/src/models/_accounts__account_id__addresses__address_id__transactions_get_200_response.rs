/*
 * Coinbase API
 *
 * The Coinbase v2 API (converted from Swagger to OpenAPI by kanekoshoyu)
 *
 * The version of the OpenAPI document: 2.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AccountsAccountIdAddressesAddressIdTransactionsGet200Response {
    #[serde(rename = "pagination", skip_serializing_if = "Option::is_none")]
    pub pagination: Option<serde_json::Value>,
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<models::Transaction>>,
}

impl AccountsAccountIdAddressesAddressIdTransactionsGet200Response {
    pub fn new() -> AccountsAccountIdAddressesAddressIdTransactionsGet200Response {
        AccountsAccountIdAddressesAddressIdTransactionsGet200Response {
            pagination: None,
            data: None,
        }
    }
}

