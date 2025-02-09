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
pub struct SelfSymbols {
    /// Response code.
    #[serde(rename = "code", skip_serializing_if = "Option::is_none")]
    pub code: Option<i32>,
    /// Array of default symbols.
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<String>>,
    /// Response message (null if successful).
    #[serde(rename = "msg", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub msg: Option<Option<String>>,
}

impl SelfSymbols {
    pub fn new() -> SelfSymbols {
        SelfSymbols {
            code: None,
            data: None,
            msg: None,
        }
    }
}

