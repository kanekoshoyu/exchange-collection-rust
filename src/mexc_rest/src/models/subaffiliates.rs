/*
 * MEXC Crypto Exchange
 *
 * Welcome to mexc API document! 
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Subaffiliates {
    #[serde(rename = "success", skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
    #[serde(rename = "code", skip_serializing_if = "Option::is_none")]
    pub code: Option<i32>,
    #[serde(rename = "message", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub message: Option<Option<String>>,
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Box<models::SubaffiliatesData>>,
}

impl Subaffiliates {
    pub fn new() -> Subaffiliates {
        Subaffiliates {
            success: None,
            code: None,
            message: None,
            data: None,
        }
    }
}

