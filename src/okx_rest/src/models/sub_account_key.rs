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
pub struct SubAccountKey {
    /// The name of the sub-account
    #[serde(rename = "subAcct", skip_serializing_if = "Option::is_none")]
    pub sub_acct: Option<String>,
    /// The public API Key for the sub-account
    #[serde(rename = "apiKey", skip_serializing_if = "Option::is_none")]
    pub api_key: Option<String>,
    /// The label associated with the sub-account API Key
    #[serde(rename = "label", skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// Permissions granted to the sub-account API Key: - `read_only`: Read - `trade`: Trade Permissions are separated by commas if there are multiple. 
    #[serde(rename = "perm", skip_serializing_if = "Option::is_none")]
    pub perm: Option<String>,
    /// IP addresses linked to the sub-account API Key. Multiple addresses are separated by commas. 
    #[serde(rename = "ip", skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    /// Creation time of the API Key in Unix timestamp format (in milliseconds).
    #[serde(rename = "ts", skip_serializing_if = "Option::is_none")]
    pub ts: Option<String>,
}

impl SubAccountKey {
    pub fn new() -> SubAccountKey {
        SubAccountKey {
            sub_acct: None,
            api_key: None,
            label: None,
            perm: None,
            ip: None,
            ts: None,
        }
    }
}

