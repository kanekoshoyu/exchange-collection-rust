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
pub struct KlineIntervalParams {
    #[serde(rename = "binary", skip_serializing_if = "Option::is_none")]
    pub binary: Option<bool>,
}

impl KlineIntervalParams {
    pub fn new() -> KlineIntervalParams {
        KlineIntervalParams {
            binary: None,
        }
    }
}

