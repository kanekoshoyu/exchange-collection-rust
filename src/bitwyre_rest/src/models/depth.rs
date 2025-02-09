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
pub struct Depth {
    #[serde(rename = "bids", skip_serializing_if = "Option::is_none")]
    pub bids: Option<Vec<Vec<String>>>,
    #[serde(rename = "asks", skip_serializing_if = "Option::is_none")]
    pub asks: Option<Vec<Vec<String>>>,
    #[serde(rename = "is_frozen", skip_serializing_if = "Option::is_none")]
    pub is_frozen: Option<i32>,
}

impl Depth {
    pub fn new() -> Depth {
        Depth {
            bids: None,
            asks: None,
            is_frozen: None,
        }
    }
}

