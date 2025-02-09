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
pub struct LeveragePref {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(rename = "maxLeverage", skip_serializing_if = "Option::is_none")]
    pub max_leverage: Option<f64>,
}

impl LeveragePref {
    pub fn new() -> LeveragePref {
        LeveragePref {
            symbol: None,
            max_leverage: None,
        }
    }
}

