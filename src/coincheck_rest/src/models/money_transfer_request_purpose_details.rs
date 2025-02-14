/*
 * Coincheck Crypto Exchange
 *
 * Welcome to Coincheck API document! 
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// MoneyTransferRequestPurposeDetails : Additional details for the specified purpose type.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct MoneyTransferRequestPurposeDetails {
    /// Specific items of goods involved in the transaction.
    #[serde(rename = "specific_items_of_goods", skip_serializing_if = "Option::is_none")]
    pub specific_items_of_goods: Option<String>,
    /// Place of origin for the goods.
    #[serde(rename = "place_of_origin", skip_serializing_if = "Option::is_none")]
    pub place_of_origin: Option<String>,
    /// Place of loading for the goods.
    #[serde(rename = "place_of_loading", skip_serializing_if = "Option::is_none")]
    pub place_of_loading: Option<String>,
    /// Place of destination for the goods (for intermediary trade).
    #[serde(rename = "place_of_destination", skip_serializing_if = "Option::is_none")]
    pub place_of_destination: Option<String>,
    /// Specific purpose of the transfer (for \"other\" purpose type).
    #[serde(rename = "extra_text", skip_serializing_if = "Option::is_none")]
    pub extra_text: Option<String>,
}

impl MoneyTransferRequestPurposeDetails {
    /// Additional details for the specified purpose type.
    pub fn new() -> MoneyTransferRequestPurposeDetails {
        MoneyTransferRequestPurposeDetails {
            specific_items_of_goods: None,
            place_of_origin: None,
            place_of_loading: None,
            place_of_destination: None,
            extra_text: None,
        }
    }
}

