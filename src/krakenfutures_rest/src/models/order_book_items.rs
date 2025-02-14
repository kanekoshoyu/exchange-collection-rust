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
pub struct OrderBookItems {
    /// The outer list is sorted ascending by ask price.
    #[serde(rename = "asks", skip_serializing_if = "Option::is_none")]
    pub asks: Option<Vec<Vec<f64>>>,
    /// The outer list is sorted descending by bid price.
    #[serde(rename = "bids", skip_serializing_if = "Option::is_none")]
    pub bids: Option<Vec<Vec<f64>>>,
}

impl OrderBookItems {
    pub fn new() -> OrderBookItems {
        OrderBookItems {
            asks: None,
            bids: None,
        }
    }
}

