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
pub struct TickersItems {
    /// The tag for the type of instrument.
    #[serde(rename = "tag", skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    /// The currency pair.
    #[serde(rename = "pair", skip_serializing_if = "Option::is_none")]
    pub pair: Option<String>,
    /// The symbol for the instrument.
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    /// The mark price of the instrument.
    #[serde(rename = "markPrice", skip_serializing_if = "Option::is_none")]
    pub mark_price: Option<f64>,
    /// The current bid price.
    #[serde(rename = "bid", skip_serializing_if = "Option::is_none")]
    pub bid: Option<f64>,
    /// The size of the current bid.
    #[serde(rename = "bidSize", skip_serializing_if = "Option::is_none")]
    pub bid_size: Option<i32>,
    /// The current ask price.
    #[serde(rename = "ask", skip_serializing_if = "Option::is_none")]
    pub ask: Option<f64>,
    /// The size of the current ask.
    #[serde(rename = "askSize", skip_serializing_if = "Option::is_none")]
    pub ask_size: Option<i32>,
    /// The volume traded in the last 24 hours.
    #[serde(rename = "vol24h", skip_serializing_if = "Option::is_none")]
    pub vol24h: Option<i32>,
    /// The open interest in the market.
    #[serde(rename = "openInterest", skip_serializing_if = "Option::is_none")]
    pub open_interest: Option<i32>,
    /// The opening price in the last 24 hours.
    #[serde(rename = "open24h", skip_serializing_if = "Option::is_none")]
    pub open24h: Option<f64>,
    /// The index price of the instrument.
    #[serde(rename = "indexPrice", skip_serializing_if = "Option::is_none")]
    pub index_price: Option<f64>,
    /// The last traded price.
    #[serde(rename = "last", skip_serializing_if = "Option::is_none")]
    pub last: Option<f64>,
    /// The timestamp of the last trade.
    #[serde(rename = "lastTime", skip_serializing_if = "Option::is_none")]
    pub last_time: Option<String>,
    /// The size of the last trade.
    #[serde(rename = "lastSize", skip_serializing_if = "Option::is_none")]
    pub last_size: Option<i32>,
    /// Whether the market is currently suspended.
    #[serde(rename = "suspended", skip_serializing_if = "Option::is_none")]
    pub suspended: Option<bool>,
    /// The current funding rate.
    #[serde(rename = "fundingRate", skip_serializing_if = "Option::is_none")]
    pub funding_rate: Option<f64>,
    /// The predicted funding rate.
    #[serde(rename = "fundingRatePrediction", skip_serializing_if = "Option::is_none")]
    pub funding_rate_prediction: Option<f64>,
    /// Whether the order is post-only.
    #[serde(rename = "postOnly", skip_serializing_if = "Option::is_none")]
    pub post_only: Option<bool>,
    /// The percentage change in price over the last 24 hours.
    #[serde(rename = "change24h", skip_serializing_if = "Option::is_none")]
    pub change24h: Option<f64>,
}

impl TickersItems {
    pub fn new() -> TickersItems {
        TickersItems {
            tag: None,
            pair: None,
            symbol: None,
            mark_price: None,
            bid: None,
            bid_size: None,
            ask: None,
            ask_size: None,
            vol24h: None,
            open_interest: None,
            open24h: None,
            index_price: None,
            last: None,
            last_time: None,
            last_size: None,
            suspended: None,
            funding_rate: None,
            funding_rate_prediction: None,
            post_only: None,
            change24h: None,
        }
    }
}

