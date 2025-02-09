#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// Trades represents a union of types: serde_json::Value, TradeItems
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(untagged)]
pub enum Trades {
    #[serde(rename="Undefined")]
    Undefined(serde_json::Value),
    #[serde(rename="TradeItems")]
    TradeItems(TradeItems),
}


