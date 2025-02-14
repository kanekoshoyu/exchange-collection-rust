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
pub struct ExchangeInfo {
    #[serde(rename = "exchangeInfo", skip_serializing_if = "Option::is_none")]
    pub exchange_info: Option<Vec<models::ExchangeInfoExchangeInfoInner>>,
}

impl ExchangeInfo {
    pub fn new() -> ExchangeInfo {
        ExchangeInfo {
            exchange_info: None,
        }
    }
}

