#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// CancelReason represents a CancelReason model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum CancelReason {
    #[serde(rename="101: Time In Force")]
    Number101ColonTimeInForce,
    #[serde(rename="102: Self Trade Prevention")]
    Number102ColonSelfTradePrevention,
    #[serde(rename="103: Admin")]
    Number103ColonAdmin,
    #[serde(rename="104: Price Bound Order Protection")]
    Number104ColonPriceBoundOrderProtection,
    #[serde(rename="105: Insufficient Funds")]
    Number105ColonInsufficientFunds,
    #[serde(rename="106: Insufficient Liquidity")]
    Number106ColonInsufficientLiquidity,
    #[serde(rename="107: Broker")]
    Number107ColonBroker,
}

