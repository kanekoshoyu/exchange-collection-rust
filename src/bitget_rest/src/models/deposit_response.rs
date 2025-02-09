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
pub struct DepositResponse {
    /// Deposit address
    #[serde(rename = "address", skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// Chain name
    #[serde(rename = "chain", skip_serializing_if = "Option::is_none")]
    pub chain: Option<String>,
    /// Token name
    #[serde(rename = "coin", skip_serializing_if = "Option::is_none")]
    pub coin: Option<String>,
    /// Tag associated with the address, required for some coins
    #[serde(rename = "tag", skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    /// URL of the blockchain address
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl DepositResponse {
    pub fn new() -> DepositResponse {
        DepositResponse {
            address: None,
            chain: None,
            coin: None,
            tag: None,
            url: None,
        }
    }
}

