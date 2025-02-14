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
pub struct Announcement {
    #[serde(rename = "unixtime", skip_serializing_if = "Option::is_none")]
    pub unixtime: Option<i32>,
    #[serde(rename = "announcement", skip_serializing_if = "Option::is_none")]
    pub announcement: Option<String>,
}

impl Announcement {
    pub fn new() -> Announcement {
        Announcement {
            unixtime: None,
            announcement: None,
        }
    }
}

