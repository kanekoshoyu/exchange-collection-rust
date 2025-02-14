/*
 * Hashkey Exchange
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
pub struct BatchOrderResponse {
    /// Error code
    #[serde(rename = "code", skip_serializing_if = "Option::is_none")]
    pub code: Option<i32>,
    #[serde(rename = "result", skip_serializing_if = "Option::is_none")]
    pub result: Option<Vec<models::BatchOrderResponseResultInner>>,
    /// Concentration reminder message
    #[serde(rename = "concentration", skip_serializing_if = "Option::is_none")]
    pub concentration: Option<String>,
}

impl BatchOrderResponse {
    pub fn new() -> BatchOrderResponse {
        BatchOrderResponse {
            code: None,
            result: None,
            concentration: None,
        }
    }
}

