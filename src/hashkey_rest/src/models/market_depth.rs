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
pub struct MarketDepth {
    /// Timestamp
    #[serde(rename = "t", skip_serializing_if = "Option::is_none")]
    pub t: Option<i64>,
    /// Buying direction details
    #[serde(rename = "b", skip_serializing_if = "Option::is_none")]
    pub b: Option<Vec<Vec<String>>>,
    /// Bid price
    #[serde(rename = "bidPrice", skip_serializing_if = "Option::is_none")]
    pub bid_price: Option<String>,
    /// Bid quantity
    #[serde(rename = "bidQuantity", skip_serializing_if = "Option::is_none")]
    pub bid_quantity: Option<String>,
    /// Selling direction details
    #[serde(rename = "sellingDirection", skip_serializing_if = "Option::is_none")]
    pub selling_direction: Option<Vec<Vec<String>>>,
    /// Ask price
    #[serde(rename = "a", skip_serializing_if = "Option::is_none")]
    pub a: Option<String>,
    /// Ask quantity
    #[serde(rename = "askQuantity", skip_serializing_if = "Option::is_none")]
    pub ask_quantity: Option<String>,
}

impl MarketDepth {
    pub fn new() -> MarketDepth {
        MarketDepth {
            t: None,
            b: None,
            bid_price: None,
            bid_quantity: None,
            selling_direction: None,
            a: None,
            ask_quantity: None,
        }
    }
}

