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
pub struct DerivativesApiV3PnlpreferencesPutRequest {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(rename = "preference", skip_serializing_if = "Option::is_none")]
    pub preference: Option<String>,
}

impl DerivativesApiV3PnlpreferencesPutRequest {
    pub fn new() -> DerivativesApiV3PnlpreferencesPutRequest {
        DerivativesApiV3PnlpreferencesPutRequest {
            symbol: None,
            preference: None,
        }
    }
}

