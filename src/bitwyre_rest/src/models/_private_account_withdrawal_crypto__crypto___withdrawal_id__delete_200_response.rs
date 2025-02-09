/*
 * Bitwyre REST API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateAccountWithdrawalCryptoCryptoWithdrawalIdDelete200Response {
    #[serde(rename = "error", skip_serializing_if = "Option::is_none")]
    pub error: Option<Vec<String>>,
    #[serde(rename = "result", skip_serializing_if = "Option::is_none")]
    pub result: Option<Box<models::CancelPendingWithdrawal>>,
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Box<models::Meta>>,
}

impl PrivateAccountWithdrawalCryptoCryptoWithdrawalIdDelete200Response {
    pub fn new() -> PrivateAccountWithdrawalCryptoCryptoWithdrawalIdDelete200Response {
        PrivateAccountWithdrawalCryptoCryptoWithdrawalIdDelete200Response {
            error: None,
            result: None,
            meta: None,
        }
    }
}

