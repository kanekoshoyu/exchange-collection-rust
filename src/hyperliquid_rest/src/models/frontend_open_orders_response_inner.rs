/*
 * Hyperliquid API
 *
 * Documentation for the Hyperliquid public API     ## **Rate limits** The following rate limits apply per IP address:   - All REST requests have a weight limit of 1200 per minute. All documented exchange API requests have a weight of 1. All documented info API requests have a weight of either 2 or 20; these limits can be found in the description for each info request in the Info endpoint section. All explorer API requests have a weight of 40.   - Maximum of 100 websocket connections   - Maximum of 1000 websocket subscriptions   - Maximum of 10 unique users across user-specific websocket subscriptions  - Maximum of 2000 inbound messages per minute across all websocket connections   - Use websockets for lowest latency realtime data. See the python SDK for a full-featured example.    ## **Address-based L1 Rate limits**    The L1 rate limiting logic will allow 1 requests per 1 USDC traded cumulatively since address inception.   Using an order value of 100 USDC, this only requires a fill rate of 1%.    Each address starts with an initial buffer of 10000 requests. When rate limited, an address will still be allowed one request every 10 seconds.  Cancels have cumulative limit min(limit + 100000, limit * 2) where limit is the default limit for other actions. This way, hitting the address-based rate limit will still allow open orders to be canceled.   Note that this rate limit only applies to L1 actions, not info requests.   ## **Batched Requests** A batched request with n orders (or cancels) is treated as one request for IP based rate limiting, but as n requests for address-based L1 rate limiting. 
 *
 * The version of the OpenAPI document: 1.0.1
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct FrontendOpenOrdersResponseInner {
    /// The cryptocurrency coin.
    #[serde(rename = "coin", skip_serializing_if = "Option::is_none")]
    pub coin: Option<String>,
    /// Whether the order is a position TPSL.
    #[serde(rename = "isPositionTpsl", skip_serializing_if = "Option::is_none")]
    pub is_position_tpsl: Option<bool>,
    /// Whether the order is a trigger order.
    #[serde(rename = "isTrigger", skip_serializing_if = "Option::is_none")]
    pub is_trigger: Option<bool>,
    /// The limit price for the order.
    #[serde(rename = "limitPx", skip_serializing_if = "Option::is_none")]
    pub limit_px: Option<String>,
    /// The order ID.
    #[serde(rename = "oid", skip_serializing_if = "Option::is_none")]
    pub oid: Option<i32>,
    /// The type of the order.
    #[serde(rename = "orderType", skip_serializing_if = "Option::is_none")]
    pub order_type: Option<String>,
    /// The original size of the order.
    #[serde(rename = "origSz", skip_serializing_if = "Option::is_none")]
    pub orig_sz: Option<String>,
    /// Whether the order is reduce-only.
    #[serde(rename = "reduceOnly", skip_serializing_if = "Option::is_none")]
    pub reduce_only: Option<bool>,
    /// The side of the order (e.g., Buy or Sell).
    #[serde(rename = "side", skip_serializing_if = "Option::is_none")]
    pub side: Option<String>,
    /// The size of the order.
    #[serde(rename = "sz", skip_serializing_if = "Option::is_none")]
    pub sz: Option<String>,
    /// The timestamp of the order.
    #[serde(rename = "timestamp", skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<i32>,
    /// The trigger condition for the order.
    #[serde(rename = "triggerCondition", skip_serializing_if = "Option::is_none")]
    pub trigger_condition: Option<String>,
    /// The trigger price for the order.
    #[serde(rename = "triggerPx", skip_serializing_if = "Option::is_none")]
    pub trigger_px: Option<serde_json::Value>,
}

impl FrontendOpenOrdersResponseInner {
    pub fn new() -> FrontendOpenOrdersResponseInner {
        FrontendOpenOrdersResponseInner {
            coin: None,
            is_position_tpsl: None,
            is_trigger: None,
            limit_px: None,
            oid: None,
            order_type: None,
            orig_sz: None,
            reduce_only: None,
            side: None,
            sz: None,
            timestamp: None,
            trigger_condition: None,
            trigger_px: None,
        }
    }
}

