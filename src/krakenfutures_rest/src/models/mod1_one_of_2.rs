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
pub struct Mod1OneOf2 {
    #[serde(rename = "ohlc", skip_serializing_if = "Option::is_none")]
    pub ohlc: Option<Vec<f64>>,
}

impl Mod1OneOf2 {
    pub fn new() -> Mod1OneOf2 {
        Mod1OneOf2 {
            ohlc: None,
        }
    }
}

