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
pub struct PlanOrderRequest {
    /// Trading pair name (e.g., BTCUSDT).
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    /// Direction of the order.
    #[serde(rename = "side", skip_serializing_if = "Option::is_none")]
    pub side: Option<Side>,
    /// Trigger price for the order.
    #[serde(rename = "triggerPrice", skip_serializing_if = "Option::is_none")]
    pub trigger_price: Option<String>,
    /// Type of order.
    #[serde(rename = "orderType", skip_serializing_if = "Option::is_none")]
    pub order_type: Option<OrderType>,
    /// Execution price (required when orderType is limit).
    #[serde(rename = "executePrice", skip_serializing_if = "Option::is_none")]
    pub execute_price: Option<String>,
    /// Order type plan.
    #[serde(rename = "planType", skip_serializing_if = "Option::is_none")]
    pub plan_type: Option<PlanType>,
    /// Quantity to buy.
    #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    /// Trigger type for the order.
    #[serde(rename = "triggerType", skip_serializing_if = "Option::is_none")]
    pub trigger_type: Option<TriggerType>,
    /// Client customized ID, only valid when orders are unfilled.
    #[serde(rename = "clientOid", skip_serializing_if = "Option::is_none")]
    pub client_oid: Option<String>,
    /// Execution strategy.
    #[serde(rename = "force", skip_serializing_if = "Option::is_none")]
    pub force: Option<Force>,
    /// STP Mode.
    #[serde(rename = "stpMode", skip_serializing_if = "Option::is_none")]
    pub stp_mode: Option<StpMode>,
}

impl PlanOrderRequest {
    pub fn new() -> PlanOrderRequest {
        PlanOrderRequest {
            symbol: None,
            side: None,
            trigger_price: None,
            order_type: None,
            execute_price: None,
            plan_type: None,
            size: None,
            trigger_type: None,
            client_oid: None,
            force: None,
            stp_mode: None,
        }
    }
}
/// Direction of the order.
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
/// Type of order.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum OrderType {
    #[serde(rename = "limit")]
    Limit,
    #[serde(rename = "market")]
    Market,
}

impl Default for OrderType {
    fn default() -> OrderType {
        Self::Limit
    }
}
/// Order type plan.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PlanType {
    #[serde(rename = "amount")]
    Amount,
    #[serde(rename = "total")]
    Total,
}

impl Default for PlanType {
    fn default() -> PlanType {
        Self::Amount
    }
}
/// Trigger type for the order.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TriggerType {
    #[serde(rename = "fill_price")]
    FillPrice,
    #[serde(rename = "mark_price")]
    MarkPrice,
}

impl Default for TriggerType {
    fn default() -> TriggerType {
        Self::FillPrice
    }
}
/// Execution strategy.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Force {
    #[serde(rename = "gtc")]
    Gtc,
    #[serde(rename = "post_only")]
    PostOnly,
    #[serde(rename = "fok")]
    Fok,
    #[serde(rename = "ioc")]
    Ioc,
}

impl Default for Force {
    fn default() -> Force {
        Self::Gtc
    }
}
/// STP Mode.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum StpMode {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "cancel_taker")]
    CancelTaker,
    #[serde(rename = "cancel_maker")]
    CancelMaker,
    #[serde(rename = "cancel_both")]
    CancelBoth,
}

impl Default for StpMode {
    fn default() -> StpMode {
        Self::None
    }
}

