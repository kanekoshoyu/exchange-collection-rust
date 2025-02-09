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
pub struct SapiV1EthStakingEthRedeemPost200Response {
    #[serde(rename = "success")]
    pub success: bool,
    #[serde(rename = "arrivalTime")]
    pub arrival_time: i64,
    #[serde(rename = "ethAmount")]
    pub eth_amount: String,
    #[serde(rename = "conversionRatio")]
    pub conversion_ratio: String,
}

impl SapiV1EthStakingEthRedeemPost200Response {
    pub fn new(success: bool, arrival_time: i64, eth_amount: String, conversion_ratio: String) -> SapiV1EthStakingEthRedeemPost200Response {
        SapiV1EthStakingEthRedeemPost200Response {
            success,
            arrival_time,
            eth_amount,
            conversion_ratio,
        }
    }
}

