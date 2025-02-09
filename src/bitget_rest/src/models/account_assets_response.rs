/*
 * Bitget API
 *
 * Welcome to Bitget Developer document! 
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AccountAssetsResponse {
    /// Token name, e.g., USDT.
    #[serde(rename = "coin", skip_serializing_if = "Option::is_none")]
    pub coin: Option<String>,
    /// Available balance of the coin.
    #[serde(rename = "available", skip_serializing_if = "Option::is_none")]
    pub available: Option<String>,
    /// Amount frozen for orders or other actions.
    #[serde(rename = "frozen", skip_serializing_if = "Option::is_none")]
    pub frozen: Option<String>,
    /// Amount locked and unavailable for trading.
    #[serde(rename = "locked", skip_serializing_if = "Option::is_none")]
    pub locked: Option<String>,
    /// Available amount within the specified limit.
    #[serde(rename = "limitAvailable", skip_serializing_if = "Option::is_none")]
    pub limit_available: Option<String>,
    /// Last update time as a Unix timestamp.
    #[serde(rename = "uTime", skip_serializing_if = "Option::is_none")]
    pub u_time: Option<String>,
}

impl AccountAssetsResponse {
    pub fn new() -> AccountAssetsResponse {
        AccountAssetsResponse {
            coin: None,
            available: None,
            frozen: None,
            locked: None,
            limit_available: None,
            u_time: None,
        }
    }
}

