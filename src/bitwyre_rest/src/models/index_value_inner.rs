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
pub struct IndexValueInner {
    #[serde(rename = "price", skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    #[serde(rename = "timestamp", skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
    #[serde(rename = "timestamp_integer", skip_serializing_if = "Option::is_none")]
    pub timestamp_integer: Option<i32>,
}

impl IndexValueInner {
    pub fn new() -> IndexValueInner {
        IndexValueInner {
            price: None,
            timestamp: None,
            timestamp_integer: None,
        }
    }
}

