#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// Union represents a union of types: String, Orderbook, serde_json::Value
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(untagged)]
pub enum Union {
    #[serde(rename="AnonymousSchema4")]
    AnonymousSchema4(String),
    #[serde(rename="Orderbook")]
    Orderbook(Orderbook),
    #[serde(rename="Undefined")]
    Undefined(serde_json::Value),
}


