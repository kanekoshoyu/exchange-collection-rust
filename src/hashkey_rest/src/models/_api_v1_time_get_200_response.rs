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
pub struct ApiV1TimeGet200Response {
    /// Server Millisecond Timestamp
    #[serde(rename = "serverTime", skip_serializing_if = "Option::is_none")]
    pub server_time: Option<i32>,
}

impl ApiV1TimeGet200Response {
    pub fn new() -> ApiV1TimeGet200Response {
        ApiV1TimeGet200Response {
            server_time: None,
        }
    }
}

