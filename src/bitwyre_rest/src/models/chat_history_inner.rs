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
pub struct ChatHistoryInner {
    #[serde(rename = "from", skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "timestamp", skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
    #[serde(rename = "to", skip_serializing_if = "Option::is_none")]
    pub to: Option<String>,
}

impl ChatHistoryInner {
    pub fn new() -> ChatHistoryInner {
        ChatHistoryInner {
            from: None,
            message: None,
            timestamp: None,
            to: None,
        }
    }
}

