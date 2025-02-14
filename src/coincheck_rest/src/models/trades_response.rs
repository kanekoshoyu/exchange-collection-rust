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
pub struct TradesResponse {
    /// Indicates if the request was successful.
    #[serde(rename = "success", skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
    #[serde(rename = "pagination", skip_serializing_if = "Option::is_none")]
    pub pagination: Option<Box<models::TradePagination>>,
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<models::TradeData>>,
}

impl TradesResponse {
    pub fn new() -> TradesResponse {
        TradesResponse {
            success: None,
            pagination: None,
            data: None,
        }
    }
}

