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
pub struct TickersResponse {
    #[serde(rename = "serverTime", skip_serializing_if = "Option::is_none")]
    pub server_time: Option<String>,
    #[serde(rename = "result", skip_serializing_if = "Option::is_none")]
    pub result: Option<String>,
    #[serde(rename = "tickers", skip_serializing_if = "Option::is_none")]
    pub tickers: Option<Vec<models::TickersItems>>,
}

impl TickersResponse {
    pub fn new() -> TickersResponse {
        TickersResponse {
            server_time: None,
            result: None,
            tickers: None,
        }
    }
}

