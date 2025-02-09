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
pub struct StatementResponse {
    #[serde(rename = "withdrawal", skip_serializing_if = "Option::is_none")]
    pub withdrawal: Option<std::collections::HashMap<String, Vec<models::StatementData>>>,
    #[serde(rename = "deposit", skip_serializing_if = "Option::is_none")]
    pub deposit: Option<std::collections::HashMap<String, Vec<models::StatementData>>>,
}

impl StatementResponse {
    pub fn new() -> StatementResponse {
        StatementResponse {
            withdrawal: None,
            deposit: None,
        }
    }
}

