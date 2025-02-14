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
pub struct ConversionListInner {
    /// The amount of MX that can be converted from the given asset.
    #[serde(rename = "convertMx", skip_serializing_if = "Option::is_none")]
    pub convert_mx: Option<String>,
    /// The amount of USDT equivalent that can be converted from the given asset.
    #[serde(rename = "convertUsdt", skip_serializing_if = "Option::is_none")]
    pub convert_usdt: Option<String>,
    /// The balance of the asset available for conversion.
    #[serde(rename = "balance", skip_serializing_if = "Option::is_none")]
    pub balance: Option<String>,
    /// The asset that can be converted into MX.
    #[serde(rename = "asset", skip_serializing_if = "Option::is_none")]
    pub asset: Option<String>,
    /// A unique code indicating the status of the conversion request.
    #[serde(rename = "code", skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// A message associated with the conversion status or error.
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl ConversionListInner {
    pub fn new() -> ConversionListInner {
        ConversionListInner {
            convert_mx: None,
            convert_usdt: None,
            balance: None,
            asset: None,
            code: None,
            message: None,
        }
    }
}

