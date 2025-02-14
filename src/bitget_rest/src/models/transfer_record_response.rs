/*
 * Bitget API
 *
 * Welcome to Bitget Developer document! 
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TransferRecordResponse {
    /// Token name (e.g., BTC)
    #[serde(rename = "coin")]
    pub coin: String,
    /// Transfer status (e.g., Successful)
    #[serde(rename = "status")]
    pub status: String,
    /// Account type to which the funds are transferred (e.g., usdt_futures)
    #[serde(rename = "toType")]
    pub to_type: String,
    /// Symbol of the account receiving the transfer, if applicable (e.g., BTC/USD)
    #[serde(rename = "toSymbol", skip_serializing_if = "Option::is_none")]
    pub to_symbol: Option<String>,
    /// Account type from which the funds are transferred (e.g., spot)
    #[serde(rename = "fromType")]
    pub from_type: String,
    /// Symbol of the account transferring the funds, if applicable (e.g., BTC/USD)
    #[serde(rename = "fromSymbol", skip_serializing_if = "Option::is_none")]
    pub from_symbol: Option<String>,
    /// Transfer amount (e.g., 1000.00000000)
    #[serde(rename = "size")]
    pub size: String,
    /// Timestamp of the transfer in Unix milliseconds (e.g., 1631070374488)
    #[serde(rename = "ts")]
    pub ts: String,
    /// Unique client order ID for the transfer
    #[serde(rename = "clientOid")]
    pub client_oid: String,
    /// Unique transfer ID
    #[serde(rename = "transferId")]
    pub transfer_id: String,
}

impl TransferRecordResponse {
    pub fn new(coin: String, status: String, to_type: String, from_type: String, size: String, ts: String, client_oid: String, transfer_id: String) -> TransferRecordResponse {
        TransferRecordResponse {
            coin,
            status,
            to_type,
            to_symbol: None,
            from_type,
            from_symbol: None,
            size,
            ts,
            client_oid,
            transfer_id,
        }
    }
}

