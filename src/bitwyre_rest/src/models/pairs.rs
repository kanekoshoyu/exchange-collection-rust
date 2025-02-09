/*
 * Bitwyre REST API
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
pub struct Pairs {
    /// The base asset code for the market pair.
    #[serde(rename = "base_asset", skip_serializing_if = "Option::is_none")]
    pub base_asset: Option<String>,
    /// The current best ask price.
    #[serde(rename = "best_ask", skip_serializing_if = "Option::is_none")]
    pub best_ask: Option<String>,
    /// The current best bid price.
    #[serde(rename = "best_bid", skip_serializing_if = "Option::is_none")]
    pub best_bid: Option<String>,
    /// The price change in the last 24 hours.
    #[serde(rename = "change_24hours", skip_serializing_if = "Option::is_none")]
    pub change_24hours: Option<String>,
    /// The country where trading this pair is restricted.
    #[serde(rename = "forbidden_country", skip_serializing_if = "Option::is_none")]
    pub forbidden_country: Option<String>,
    /// The minimum price increment for orders.
    #[serde(rename = "incremental_price", skip_serializing_if = "Option::is_none")]
    pub incremental_price: Option<String>,
    /// The full name of the trading instrument (base and quote pair with market).
    #[serde(rename = "instrument", skip_serializing_if = "Option::is_none")]
    pub instrument: Option<String>,
    /// Indicates if trading is halted for the instrument.
    #[serde(rename = "is_halt", skip_serializing_if = "Option::is_none")]
    pub is_halt: Option<bool>,
    /// Indicates if the matching engine is replaying trades.
    #[serde(rename = "is_matching_engine_replaying", skip_serializing_if = "Option::is_none")]
    pub is_matching_engine_replaying: Option<bool>,
    /// The last traded price.
    #[serde(rename = "last", skip_serializing_if = "Option::is_none")]
    pub last: Option<String>,
    /// The market type (e.g., crypto).
    #[serde(rename = "market", skip_serializing_if = "Option::is_none")]
    pub market: Option<String>,
    /// The minimum order size for the market pair.
    #[serde(rename = "minimum_order_size", skip_serializing_if = "Option::is_none")]
    pub minimum_order_size: Option<String>,
    /// The product type (e.g., spot).
    #[serde(rename = "product", skip_serializing_if = "Option::is_none")]
    pub product: Option<String>,
    /// The quote asset code for the market pair.
    #[serde(rename = "quote_asset", skip_serializing_if = "Option::is_none")]
    pub quote_asset: Option<String>,
    /// The minimum size increment for orders.
    #[serde(rename = "size_increment", skip_serializing_if = "Option::is_none")]
    pub size_increment: Option<String>,
    /// The symbol representing the market pair (base/quote).
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    /// The trading volume in the base asset over the last 24 hours.
    #[serde(rename = "volume_base", skip_serializing_if = "Option::is_none")]
    pub volume_base: Option<String>,
    /// The trading volume in the quote asset over the last 24 hours.
    #[serde(rename = "volume_quote", skip_serializing_if = "Option::is_none")]
    pub volume_quote: Option<String>,
}

impl Pairs {
    pub fn new() -> Pairs {
        Pairs {
            base_asset: None,
            best_ask: None,
            best_bid: None,
            change_24hours: None,
            forbidden_country: None,
            incremental_price: None,
            instrument: None,
            is_halt: None,
            is_matching_engine_replaying: None,
            last: None,
            market: None,
            minimum_order_size: None,
            product: None,
            quote_asset: None,
            size_increment: None,
            symbol: None,
            volume_base: None,
            volume_quote: None,
        }
    }
}

