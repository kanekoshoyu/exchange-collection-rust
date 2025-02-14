/*
 * Coincheck Crypto Exchange
 *
 * Welcome to Coincheck API document! 
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExchangeStatusAvailability {
    /// Whether limit orders can be placed.
    #[serde(rename = "order", skip_serializing_if = "Option::is_none")]
    pub order: Option<bool>,
    /// Whether market orders can be placed.
    #[serde(rename = "market_order", skip_serializing_if = "Option::is_none")]
    pub market_order: Option<bool>,
    /// Whether cancel orders can be placed.
    #[serde(rename = "cancel", skip_serializing_if = "Option::is_none")]
    pub cancel: Option<bool>,
}

impl ExchangeStatusAvailability {
    pub fn new() -> ExchangeStatusAvailability {
        ExchangeStatusAvailability {
            order: None,
            market_order: None,
            cancel: None,
        }
    }
}

