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
pub struct Candle {
    /// Epoch time in milliseconds.
    #[serde(rename = "time")]
    pub time: i64,
    /// The highest price during the period.
    #[serde(rename = "high")]
    pub high: String,
    /// The lowest price during the period.
    #[serde(rename = "low")]
    pub low: String,
    /// The opening price at the beginning of the period.
    #[serde(rename = "open")]
    pub open: String,
    /// The closing price at the end of the period.
    #[serde(rename = "close")]
    pub close: String,
    /// The trading volume during the period.
    #[serde(rename = "volume")]
    pub volume: i64,
}

impl Candle {
    pub fn new(time: i64, high: String, low: String, open: String, close: String, volume: i64) -> Candle {
        Candle {
            time,
            high,
            low,
            open,
            close,
            volume,
        }
    }
}

