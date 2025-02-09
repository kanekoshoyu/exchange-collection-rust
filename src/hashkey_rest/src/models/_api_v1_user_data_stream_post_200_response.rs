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
pub struct ApiV1UserDataStreamPost200Response {
    /// Key to subscribe websocket feeds
    #[serde(rename = "listenkey", skip_serializing_if = "Option::is_none")]
    pub listenkey: Option<String>,
}

impl ApiV1UserDataStreamPost200Response {
    pub fn new() -> ApiV1UserDataStreamPost200Response {
        ApiV1UserDataStreamPost200Response {
            listenkey: None,
        }
    }
}

