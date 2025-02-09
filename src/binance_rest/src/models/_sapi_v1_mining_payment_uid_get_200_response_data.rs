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
pub struct SapiV1MiningPaymentUidGet200ResponseData {
    #[serde(rename = "accountProfits")]
    pub account_profits: Vec<models::SapiV1MiningPaymentUidGet200ResponseDataAccountProfitsInner>,
    #[serde(rename = "totalNum")]
    pub total_num: i32,
    #[serde(rename = "pageSize")]
    pub page_size: i32,
}

impl SapiV1MiningPaymentUidGet200ResponseData {
    pub fn new(account_profits: Vec<models::SapiV1MiningPaymentUidGet200ResponseDataAccountProfitsInner>, total_num: i32, page_size: i32) -> SapiV1MiningPaymentUidGet200ResponseData {
        SapiV1MiningPaymentUidGet200ResponseData {
            account_profits,
            total_num,
            page_size,
        }
    }
}

