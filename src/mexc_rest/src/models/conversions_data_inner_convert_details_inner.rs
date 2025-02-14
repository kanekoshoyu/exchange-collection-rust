/*
 * MEXC Crypto Exchange
 *
 * Welcome to mexc API document! 
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConversionsDataInnerConvertDetailsInner {
    /// The unique identifier for this conversion transaction.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The amount converted.
    #[serde(rename = "convert", skip_serializing_if = "Option::is_none")]
    pub convert: Option<String>,
    /// The fee for this particular conversion.
    #[serde(rename = "fee", skip_serializing_if = "Option::is_none")]
    pub fee: Option<String>,
    /// The remaining amount after fee.
    #[serde(rename = "amount", skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
    /// The timestamp of the individual conversion.
    #[serde(rename = "time", skip_serializing_if = "Option::is_none")]
    pub time: Option<i32>,
    /// The asset being converted.
    #[serde(rename = "asset", skip_serializing_if = "Option::is_none")]
    pub asset: Option<String>,
}

impl ConversionsDataInnerConvertDetailsInner {
    pub fn new() -> ConversionsDataInnerConvertDetailsInner {
        ConversionsDataInnerConvertDetailsInner {
            id: None,
            convert: None,
            fee: None,
            amount: None,
            time: None,
            asset: None,
        }
    }
}

