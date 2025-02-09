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
pub struct ConversionsInner {
    #[serde(rename = "timestamp", skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
    #[serde(rename = "base_asset", skip_serializing_if = "Option::is_none")]
    pub base_asset: Option<String>,
    #[serde(rename = "convert_uuid", skip_serializing_if = "Option::is_none")]
    pub convert_uuid: Option<uuid::Uuid>,
    #[serde(rename = "quote_asset", skip_serializing_if = "Option::is_none")]
    pub quote_asset: Option<String>,
    #[serde(rename = "details", skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
    #[serde(rename = "expired", skip_serializing_if = "Option::is_none")]
    pub expired: Option<bool>,
    #[serde(rename = "accepted", skip_serializing_if = "Option::is_none")]
    pub accepted: Option<bool>,
    #[serde(rename = "price", skip_serializing_if = "Option::is_none")]
    pub price: Option<f32>,
    #[serde(rename = "proceeds", skip_serializing_if = "Option::is_none")]
    pub proceeds: Option<f32>,
    #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
    pub size: Option<f32>,
    #[serde(rename = "side", skip_serializing_if = "Option::is_none")]
    pub side: Option<i32>,
    #[serde(rename = "success_time", skip_serializing_if = "Option::is_none")]
    pub success_time: Option<String>,
}

impl ConversionsInner {
    pub fn new() -> ConversionsInner {
        ConversionsInner {
            timestamp: None,
            base_asset: None,
            convert_uuid: None,
            quote_asset: None,
            details: None,
            expired: None,
            accepted: None,
            price: None,
            proceeds: None,
            size: None,
            side: None,
            success_time: None,
        }
    }
}

