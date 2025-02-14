/*
 * MEXC Crypto Exchange
 *
 * Welcome to mexc API document! 
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SubaccountsSubAccountsInner {
    /// The name of the sub-account.
    #[serde(rename = "subAccount", skip_serializing_if = "Option::is_none")]
    pub sub_account: Option<String>,
    /// Indicates whether the sub-account is frozen.
    #[serde(rename = "isFreeze", skip_serializing_if = "Option::is_none")]
    pub is_freeze: Option<bool>,
    /// The creation time of the sub-account in milliseconds since the epoch.
    #[serde(rename = "createTime", skip_serializing_if = "Option::is_none")]
    pub create_time: Option<i32>,
    /// The unique user ID associated with the sub-account.
    #[serde(rename = "uid", skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
}

impl SubaccountsSubAccountsInner {
    pub fn new() -> SubaccountsSubAccountsInner {
        SubaccountsSubAccountsInner {
            sub_account: None,
            is_freeze: None,
            create_time: None,
            uid: None,
        }
    }
}

