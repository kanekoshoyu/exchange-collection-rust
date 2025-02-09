/*
 * Hashkey Exchange
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
pub struct Coin {
    /// Institution ID
    #[serde(rename = "orgId", skip_serializing_if = "Option::is_none")]
    pub org_id: Option<String>,
    /// Coin ID
    #[serde(rename = "coinId", skip_serializing_if = "Option::is_none")]
    pub coin_id: Option<String>,
    /// Coin name
    #[serde(rename = "coinName", skip_serializing_if = "Option::is_none")]
    pub coin_name: Option<String>,
    /// Whether to allow withdrawal
    #[serde(rename = "allowWithdraw", skip_serializing_if = "Option::is_none")]
    pub allow_withdraw: Option<bool>,
    /// Whether to allow deposit
    #[serde(rename = "allowDeposit", skip_serializing_if = "Option::is_none")]
    pub allow_deposit: Option<bool>,
}

impl Coin {
    pub fn new() -> Coin {
        Coin {
            org_id: None,
            coin_id: None,
            coin_name: None,
            allow_withdraw: None,
            allow_deposit: None,
        }
    }
}

