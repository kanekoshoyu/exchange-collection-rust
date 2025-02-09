/*
 * Kraken API
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
pub struct OrderBookData {
    #[serde(rename = "bid", skip_serializing_if = "Option::is_none")]
    pub bid: Option<Box<models::OrderBookDataBid>>,
    #[serde(rename = "ask", skip_serializing_if = "Option::is_none")]
    pub ask: Option<Box<models::OrderBookDataBid>>,
}

impl OrderBookData {
    pub fn new() -> OrderBookData {
        OrderBookData {
            bid: None,
            ask: None,
        }
    }
}

