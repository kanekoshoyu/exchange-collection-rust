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
pub struct Leverage {
    #[serde(rename = "current_leverage", skip_serializing_if = "Option::is_none")]
    pub current_leverage: Option<i32>,
    #[serde(rename = "instrument", skip_serializing_if = "Option::is_none")]
    pub instrument: Option<String>,
}

impl Leverage {
    pub fn new() -> Leverage {
        Leverage {
            current_leverage: None,
            instrument: None,
        }
    }
}

