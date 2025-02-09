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
pub struct CoinInfo {
    #[serde(rename = "coinId", skip_serializing_if = "Option::is_none")]
    pub coin_id: Option<String>,
    #[serde(rename = "coin", skip_serializing_if = "Option::is_none")]
    pub coin: Option<String>,
    #[serde(rename = "transfer", skip_serializing_if = "Option::is_none")]
    pub transfer: Option<bool>,
    #[serde(rename = "chains", skip_serializing_if = "Option::is_none")]
    pub chains: Option<Vec<models::Chain>>,
}

impl CoinInfo {
    pub fn new() -> CoinInfo {
        CoinInfo {
            coin_id: None,
            coin: None,
            transfer: None,
            chains: None,
        }
    }
}

