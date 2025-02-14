/*
 * Kraken API
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
pub struct SubaccountsResponse {
    /// Server time in Coordinated Universal Time (UTC)
    #[serde(rename = "serverTime", skip_serializing_if = "Option::is_none")]
    pub server_time: Option<String>,
    /// Possible values for the result of the request
    #[serde(rename = "result", skip_serializing_if = "Option::is_none")]
    pub result: Option<Result>,
    /// Master account UID
    #[serde(rename = "masterAccountUid", skip_serializing_if = "Option::is_none")]
    pub master_account_uid: Option<String>,
    #[serde(rename = "subaccounts", skip_serializing_if = "Option::is_none")]
    pub subaccounts: Option<Vec<models::SubAccountsArray>>,
}

impl SubaccountsResponse {
    pub fn new() -> SubaccountsResponse {
        SubaccountsResponse {
            server_time: None,
            result: None,
            master_account_uid: None,
            subaccounts: None,
        }
    }
}
/// Possible values for the result of the request
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Result {
    #[serde(rename = "success")]
    Success,
}

impl Default for Result {
    fn default() -> Result {
        Self::Success
    }
}

