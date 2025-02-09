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
pub struct PoolError {
    /// Severity of the error
    #[serde(rename = "severity")]
    pub severity: String,
    /// Class of the error
    #[serde(rename = "error_class")]
    pub error_class: String,
    /// Type of the error
    #[serde(rename = "type")]
    pub r#type: String,
    /// Message associated with the error
    #[serde(rename = "msg")]
    pub msg: String,
    /// Value related to the error
    #[serde(rename = "value")]
    pub value: String,
    /// Field related to the error
    #[serde(rename = "field")]
    pub field: String,
}

impl PoolError {
    pub fn new(severity: String, error_class: String, r#type: String, msg: String, value: String, field: String) -> PoolError {
        PoolError {
            severity,
            error_class,
            r#type,
            msg,
            value,
            field,
        }
    }
}

