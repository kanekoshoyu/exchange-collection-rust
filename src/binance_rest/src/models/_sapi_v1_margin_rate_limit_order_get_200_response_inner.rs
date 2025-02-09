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
pub struct SapiV1MarginRateLimitOrderGet200ResponseInner {
    #[serde(rename = "rateLimitType")]
    pub rate_limit_type: String,
    #[serde(rename = "interval")]
    pub interval: String,
    #[serde(rename = "intervalNum")]
    pub interval_num: i64,
    #[serde(rename = "limit")]
    pub limit: i64,
    #[serde(rename = "count")]
    pub count: i64,
}

impl SapiV1MarginRateLimitOrderGet200ResponseInner {
    pub fn new(rate_limit_type: String, interval: String, interval_num: i64, limit: i64, count: i64) -> SapiV1MarginRateLimitOrderGet200ResponseInner {
        SapiV1MarginRateLimitOrderGet200ResponseInner {
            rate_limit_type,
            interval,
            interval_num,
            limit,
            count,
        }
    }
}

