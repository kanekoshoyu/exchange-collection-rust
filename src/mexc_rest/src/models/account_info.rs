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
pub struct AccountInfo {
    #[serde(rename = "canTrade", skip_serializing_if = "Option::is_none")]
    pub can_trade: Option<bool>,
    #[serde(rename = "canWithdraw", skip_serializing_if = "Option::is_none")]
    pub can_withdraw: Option<bool>,
    #[serde(rename = "canDeposit", skip_serializing_if = "Option::is_none")]
    pub can_deposit: Option<bool>,
    #[serde(rename = "updateTime", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub update_time: Option<Option<String>>,
    #[serde(rename = "accountType", skip_serializing_if = "Option::is_none")]
    pub account_type: Option<String>,
    #[serde(rename = "balances", skip_serializing_if = "Option::is_none")]
    pub balances: Option<Vec<models::AccountInfoBalancesInner>>,
    #[serde(rename = "permissions", skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<String>>,
}

impl AccountInfo {
    pub fn new() -> AccountInfo {
        AccountInfo {
            can_trade: None,
            can_withdraw: None,
            can_deposit: None,
            update_time: None,
            account_type: None,
            balances: None,
            permissions: None,
        }
    }
}

