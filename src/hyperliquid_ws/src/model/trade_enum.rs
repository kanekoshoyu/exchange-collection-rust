#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// TradeEnum represents a TradeEnum model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum TradeEnum {
    #[serde(rename="trades")]
    Trades,
}

