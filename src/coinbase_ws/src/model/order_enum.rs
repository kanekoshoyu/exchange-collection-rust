#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// OrderEnum represents a OrderEnum model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum OrderEnum {
    #[serde(rename="LIMIT_ORDER_TYPE")]
    LimitOrderType,
    #[serde(rename="MARKET_ORDER_TYPE")]
    MarketOrderType,
    #[serde(rename="STOP_LIMIT_ORDER_TYPE")]
    StopLimitOrderType,
}

