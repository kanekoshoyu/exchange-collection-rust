/*
 * MEXC Crypto Exchange
 *
 * Welcome to mexc API document! 
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Transfer {
    #[serde(rename = "rows", skip_serializing_if = "Option::is_none")]
    pub rows: Option<Vec<models::TransferRowsInner>>,
    /// The total number of records available.
    #[serde(rename = "total", skip_serializing_if = "Option::is_none")]
    pub total: Option<i32>,
}

impl Transfer {
    pub fn new() -> Transfer {
        Transfer {
            rows: None,
            total: None,
        }
    }
}

