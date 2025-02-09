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
pub struct DerivativesApiV3FillsGet200Response {
    #[serde(rename = "serverTime", skip_serializing_if = "Option::is_none")]
    pub server_time: Option<String>,
    #[serde(rename = "result", skip_serializing_if = "Option::is_none")]
    pub result: Option<String>,
    #[serde(rename = "fills", skip_serializing_if = "Option::is_none")]
    pub fills: Option<Vec<models::Fills>>,
}

impl DerivativesApiV3FillsGet200Response {
    pub fn new() -> DerivativesApiV3FillsGet200Response {
        DerivativesApiV3FillsGet200Response {
            server_time: None,
            result: None,
            fills: None,
        }
    }
}

