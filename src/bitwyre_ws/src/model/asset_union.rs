#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// AssetUnion represents a union of types: serde_json::Value, AssetBalance
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(untagged)]
pub enum AssetUnion {
    #[serde(rename="Undefined")]
    Undefined(serde_json::Value),
    #[serde(rename="AssetBalance")]
    AssetBalance(AssetBalance),
}


