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
pub struct SapiV1LendingAutoInvestOneOffPostDetailsParameterInner {
    #[serde(rename = "targetAsset", skip_serializing_if = "Option::is_none")]
    pub target_asset: Option<String>,
    #[serde(rename = "percentage", skip_serializing_if = "Option::is_none")]
    pub percentage: Option<i32>,
}

impl SapiV1LendingAutoInvestOneOffPostDetailsParameterInner {
    pub fn new() -> SapiV1LendingAutoInvestOneOffPostDetailsParameterInner {
        SapiV1LendingAutoInvestOneOffPostDetailsParameterInner {
            target_asset: None,
            percentage: None,
        }
    }
}

