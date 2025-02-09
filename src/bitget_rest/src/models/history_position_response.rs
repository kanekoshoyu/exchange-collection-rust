/*
 * Bitget API
 *
 * Welcome to Bitget Developer document! 
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct HistoryPositionResponse {
    #[serde(rename = "list", skip_serializing_if = "Option::is_none")]
    pub list: Option<Vec<models::Position>>,
    /// The end ID for pagination, indicating older data.
    #[serde(rename = "endId", skip_serializing_if = "Option::is_none")]
    pub end_id: Option<String>,
}

impl HistoryPositionResponse {
    pub fn new() -> HistoryPositionResponse {
        HistoryPositionResponse {
            list: None,
            end_id: None,
        }
    }
}

