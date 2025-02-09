/*
 * Binance Public Spot API
 *
 * OpenAPI Specifications for the Binance Public Spot API  API documents:   - [https://github.com/binance/binance-spot-api-docs](https://github.com/binance/binance-spot-api-docs)   - [https://binance-docs.github.io/apidocs/spot/en](https://binance-docs.github.io/apidocs/spot/en)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SapiV1SubAccountSubAccountApiIpRestrictionGet200Response {
    #[serde(rename = "ipRestrict")]
    pub ip_restrict: String,
    #[serde(rename = "ipList")]
    pub ip_list: Vec<String>,
    #[serde(rename = "updateTime")]
    pub update_time: i64,
    #[serde(rename = "apiKey")]
    pub api_key: String,
}

impl SapiV1SubAccountSubAccountApiIpRestrictionGet200Response {
    pub fn new(ip_restrict: String, ip_list: Vec<String>, update_time: i64, api_key: String) -> SapiV1SubAccountSubAccountApiIpRestrictionGet200Response {
        SapiV1SubAccountSubAccountApiIpRestrictionGet200Response {
            ip_restrict,
            ip_list,
            update_time,
            api_key,
        }
    }
}

