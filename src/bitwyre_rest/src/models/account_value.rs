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
pub struct AccountValue {
    #[serde(rename = "usd", skip_serializing_if = "Option::is_none")]
    pub usd: Option<f32>,
    #[serde(rename = "idr", skip_serializing_if = "Option::is_none")]
    pub idr: Option<f32>,
}

impl AccountValue {
    pub fn new() -> AccountValue {
        AccountValue {
            usd: None,
            idr: None,
        }
    }
}

