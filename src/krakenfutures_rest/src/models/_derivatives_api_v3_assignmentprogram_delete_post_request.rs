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
pub struct DerivativesApiV3AssignmentprogramDeletePostRequest {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

impl DerivativesApiV3AssignmentprogramDeletePostRequest {
    pub fn new() -> DerivativesApiV3AssignmentprogramDeletePostRequest {
        DerivativesApiV3AssignmentprogramDeletePostRequest {
            id: None,
        }
    }
}

