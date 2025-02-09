/*
 * OKX API
 *
 * Welcome to OKX Developer document!   excluded below endpoints as they are if you need them please add and commit to https://github.com/kanekoshoyu/exchange-collection): - Trading Account (this might become needed, will add when we need it) - Block Trading - Financial Producer - Affiliate - Status - Announcement 
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct EstimationResponse {
    /// Quotation generation time, Unix timestamp format in milliseconds
    #[serde(rename = "quoteTime")]
    pub quote_time: String,
    /// Validity period of quotation in milliseconds
    #[serde(rename = "ttlMs")]
    pub ttl_ms: String,
    /// Client Order ID as assigned by the client
    #[serde(rename = "clQReqId")]
    pub cl_q_req_id: String,
    /// Quote ID
    #[serde(rename = "quoteId")]
    pub quote_id: String,
    /// Base currency, e.g., BTC in BTC-USDT
    #[serde(rename = "baseCcy")]
    pub base_ccy: String,
    /// Quote currency, e.g., USDT in BTC-USDT
    #[serde(rename = "quoteCcy")]
    pub quote_ccy: String,
    /// Trade side based on baseCcy
    #[serde(rename = "side")]
    pub side: Side,
    /// Original RFQ amount
    #[serde(rename = "origRfqSz")]
    pub orig_rfq_sz: String,
    /// Real RFQ amount
    #[serde(rename = "rfqSz")]
    pub rfq_sz: String,
    /// RFQ currency
    #[serde(rename = "rfqSzCcy")]
    pub rfq_sz_ccy: String,
    /// Convert price based on quote currency
    #[serde(rename = "cnvtPx")]
    pub cnvt_px: String,
    /// Convert amount of base currency
    #[serde(rename = "baseSz")]
    pub base_sz: String,
    /// Convert amount of quote currency
    #[serde(rename = "quoteSz")]
    pub quote_sz: String,
}

impl EstimationResponse {
    pub fn new(quote_time: String, ttl_ms: String, cl_q_req_id: String, quote_id: String, base_ccy: String, quote_ccy: String, side: Side, orig_rfq_sz: String, rfq_sz: String, rfq_sz_ccy: String, cnvt_px: String, base_sz: String, quote_sz: String) -> EstimationResponse {
        EstimationResponse {
            quote_time,
            ttl_ms,
            cl_q_req_id,
            quote_id,
            base_ccy,
            quote_ccy,
            side,
            orig_rfq_sz,
            rfq_sz,
            rfq_sz_ccy,
            cnvt_px,
            base_sz,
            quote_sz,
        }
    }
}
/// Trade side based on baseCcy
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Side {
    #[serde(rename = "buy")]
    Buy,
    #[serde(rename = "sell")]
    Sell,
}

impl Default for Side {
    fn default() -> Side {
        Self::Buy
    }
}

