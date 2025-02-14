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
pub struct FutureAccountsResponseAssetListInner {
    #[serde(rename = "coin", skip_serializing_if = "Option::is_none")]
    pub coin: Option<String>,
    #[serde(rename = "balance", skip_serializing_if = "Option::is_none")]
    pub balance: Option<String>,
}

impl FutureAccountsResponseAssetListInner {
    pub fn new() -> FutureAccountsResponseAssetListInner {
        FutureAccountsResponseAssetListInner {
            coin: None,
            balance: None,
        }
    }
}

