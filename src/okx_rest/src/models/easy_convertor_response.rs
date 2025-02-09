/*
 * OKX API
 *
 * Welcome to OKX Developer document!   excluded below endpoints as they are if you need them please add and commit to https://github.com/kanekoshoyu/exchange-collection): - Trading Account (this might become needed, will add when we need it) - Block Trading - Financial Producer - Affiliate - Status - Announcement 
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct EasyConvertorResponse {
    /// The amount of the currency converted from.
    #[serde(rename = "fillFromSz", skip_serializing_if = "Option::is_none")]
    pub fill_from_sz: Option<String>,
    /// The amount of the currency converted to.
    #[serde(rename = "fillToSz", skip_serializing_if = "Option::is_none")]
    pub fill_to_sz: Option<String>,
    /// The currency being converted from.
    #[serde(rename = "fromCcy", skip_serializing_if = "Option::is_none")]
    pub from_ccy: Option<String>,
    /// The current status of the conversion.
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    /// The currency being converted to.
    #[serde(rename = "toCcy", skip_serializing_if = "Option::is_none")]
    pub to_ccy: Option<String>,
    /// The last update time of the conversion in Unix timestamp format (milliseconds).
    #[serde(rename = "uTime", skip_serializing_if = "Option::is_none")]
    pub u_time: Option<String>,
}

impl EasyConvertorResponse {
    pub fn new() -> EasyConvertorResponse {
        EasyConvertorResponse {
            fill_from_sz: None,
            fill_to_sz: None,
            from_ccy: None,
            status: None,
            to_ccy: None,
            u_time: None,
        }
    }
}
/// The current status of the conversion.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "running")]
    Running,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "failed")]
    Failed,
}

impl Default for Status {
    fn default() -> Status {
        Self::Running
    }
}

