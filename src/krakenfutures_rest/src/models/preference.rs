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
pub struct Preference {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(rename = "pnlCurrency", skip_serializing_if = "Option::is_none")]
    pub pnl_currency: Option<String>,
}

impl Preference {
    pub fn new() -> Preference {
        Preference {
            symbol: None,
            pnl_currency: None,
        }
    }
}

