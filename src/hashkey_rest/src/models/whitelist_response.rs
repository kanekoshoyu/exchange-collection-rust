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
pub struct WhitelistResponse {
    /// Whitelisted address.
    #[serde(rename = "fromAddress", skip_serializing_if = "Option::is_none")]
    pub from_address: Option<String>,
    /// Coin name.
    #[serde(rename = "coin", skip_serializing_if = "Option::is_none")]
    pub coin: Option<String>,
    /// Coin chain type.
    #[serde(rename = "chainType", skip_serializing_if = "Option::is_none")]
    pub chain_type: Option<ChainType>,
}

impl WhitelistResponse {
    pub fn new() -> WhitelistResponse {
        WhitelistResponse {
            from_address: None,
            coin: None,
            chain_type: None,
        }
    }
}
/// Coin chain type.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ChainType {
    #[serde(rename = "USDT_ETH")]
    UsdtEth,
    #[serde(rename = "USDT_BTC")]
    UsdtBtc,
}

impl Default for ChainType {
    fn default() -> ChainType {
        Self::UsdtEth
    }
}

