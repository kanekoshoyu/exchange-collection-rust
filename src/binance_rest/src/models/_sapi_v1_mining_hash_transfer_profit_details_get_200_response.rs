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
pub struct SapiV1MiningHashTransferProfitDetailsGet200Response {
    #[serde(rename = "code")]
    pub code: i64,
    #[serde(rename = "msg")]
    pub msg: String,
    #[serde(rename = "data")]
    pub data: Box<models::SapiV1MiningHashTransferProfitDetailsGet200ResponseData>,
}

impl SapiV1MiningHashTransferProfitDetailsGet200Response {
    pub fn new(code: i64, msg: String, data: models::SapiV1MiningHashTransferProfitDetailsGet200ResponseData) -> SapiV1MiningHashTransferProfitDetailsGet200Response {
        SapiV1MiningHashTransferProfitDetailsGet200Response {
            code,
            msg,
            data: Box::new(data),
        }
    }
}

