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
pub struct OpenOrderResponse {
    /// Server time in Coordinated Universal Time (UTC).
    #[serde(rename = "serverTime", skip_serializing_if = "Option::is_none")]
    pub server_time: Option<String>,
    /// The result of the cancellation operation. Always \"success\".
    #[serde(rename = "result", skip_serializing_if = "Option::is_none")]
    pub result: Option<Result>,
    #[serde(rename = "openOrders", skip_serializing_if = "Option::is_none")]
    pub open_orders: Option<Vec<models::Order>>,
}

impl OpenOrderResponse {
    pub fn new() -> OpenOrderResponse {
        OpenOrderResponse {
            server_time: None,
            result: None,
            open_orders: None,
        }
    }
}
/// The result of the cancellation operation. Always \"success\".
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Result {
    #[serde(rename = "success")]
    Success,
}

impl Default for Result {
    fn default() -> Result {
        Self::Success
    }
}

