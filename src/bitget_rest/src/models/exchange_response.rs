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
pub struct ExchangeResponse {
    /// The coin name (e.g., BTC)
    #[serde(rename = "coin")]
    pub coin: String,
    /// A list of exchange rates for the specified coin
    #[serde(rename = "exchangeRateList")]
    pub exchange_rate_list: Vec<models::ExchangeResponseExchangeRateListInner>,
}

impl ExchangeResponse {
    pub fn new(coin: String, exchange_rate_list: Vec<models::ExchangeResponseExchangeRateListInner>) -> ExchangeResponse {
        ExchangeResponse {
            coin,
            exchange_rate_list,
        }
    }
}

