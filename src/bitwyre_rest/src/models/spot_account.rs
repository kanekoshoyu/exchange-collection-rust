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
pub struct SpotAccount {
    #[serde(rename = "asset", skip_serializing_if = "Option::is_none")]
    pub asset: Option<String>,
    #[serde(rename = "available_balance", skip_serializing_if = "Option::is_none")]
    pub available_balance: Option<f32>,
    #[serde(rename = "constant_multiplier", skip_serializing_if = "Option::is_none")]
    pub constant_multiplier: Option<i32>,
    #[serde(rename = "icon_url", skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    #[serde(rename = "is_deposit_enabled", skip_serializing_if = "Option::is_none")]
    pub is_deposit_enabled: Option<bool>,
    #[serde(rename = "is_trading_enabled", skip_serializing_if = "Option::is_none")]
    pub is_trading_enabled: Option<bool>,
    #[serde(rename = "is_withdraw_enabled", skip_serializing_if = "Option::is_none")]
    pub is_withdraw_enabled: Option<bool>,
    #[serde(rename = "btc_equivalent", skip_serializing_if = "Option::is_none")]
    pub btc_equivalent: Option<i32>,
    #[serde(rename = "local_equivalent", skip_serializing_if = "Option::is_none")]
    pub local_equivalent: Option<i32>,
    #[serde(rename = "local_reference", skip_serializing_if = "Option::is_none")]
    pub local_reference: Option<String>,
    #[serde(rename = "locked_balance", skip_serializing_if = "Option::is_none")]
    pub locked_balance: Option<i32>,
    #[serde(rename = "max_withdrawal", skip_serializing_if = "Option::is_none")]
    pub max_withdrawal: Option<String>,
    #[serde(rename = "min_withdrawal", skip_serializing_if = "Option::is_none")]
    pub min_withdrawal: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "precision", skip_serializing_if = "Option::is_none")]
    pub precision: Option<String>,
    #[serde(rename = "total_balance", skip_serializing_if = "Option::is_none")]
    pub total_balance: Option<i32>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "usd_equivalent", skip_serializing_if = "Option::is_none")]
    pub usd_equivalent: Option<i32>,
    #[serde(rename = "withdrawal_fee", skip_serializing_if = "Option::is_none")]
    pub withdrawal_fee: Option<String>,
    #[serde(rename = "is_usd_stablecoin", skip_serializing_if = "Option::is_none")]
    pub is_usd_stablecoin: Option<bool>,
}

impl SpotAccount {
    pub fn new() -> SpotAccount {
        SpotAccount {
            asset: None,
            available_balance: None,
            constant_multiplier: None,
            icon_url: None,
            is_deposit_enabled: None,
            is_trading_enabled: None,
            is_withdraw_enabled: None,
            btc_equivalent: None,
            local_equivalent: None,
            local_reference: None,
            locked_balance: None,
            max_withdrawal: None,
            min_withdrawal: None,
            name: None,
            precision: None,
            total_balance: None,
            r#type: None,
            usd_equivalent: None,
            withdrawal_fee: None,
            is_usd_stablecoin: None,
        }
    }
}

