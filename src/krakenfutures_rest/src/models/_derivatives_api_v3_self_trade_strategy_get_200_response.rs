/*
 * Kraken API
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
pub struct DerivativesApiV3SelfTradeStrategyGet200Response {
    #[serde(rename = "serverTime", skip_serializing_if = "Option::is_none")]
    pub server_time: Option<String>,
    #[serde(rename = "result", skip_serializing_if = "Option::is_none")]
    pub result: Option<String>,
    #[serde(rename = "strategy", skip_serializing_if = "Option::is_none")]
    pub strategy: Option<Strategy>,
}

impl DerivativesApiV3SelfTradeStrategyGet200Response {
    pub fn new() -> DerivativesApiV3SelfTradeStrategyGet200Response {
        DerivativesApiV3SelfTradeStrategyGet200Response {
            server_time: None,
            result: None,
            strategy: None,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Strategy {
    #[serde(rename = "REJECT_MAKER")]
    RejectMaker,
    #[serde(rename = "CANCEL_MAKER_SELF")]
    CancelMakerSelf,
    #[serde(rename = "CANCEL_MAKER_CHILD")]
    CancelMakerChild,
    #[serde(rename = "CANCEL_MAKER_ANY")]
    CancelMakerAny,
}

impl Default for Strategy {
    fn default() -> Strategy {
        Self::RejectMaker
    }
}

