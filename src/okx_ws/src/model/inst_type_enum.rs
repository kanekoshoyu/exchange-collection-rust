#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// InstTypeEnum represents a InstTypeEnum model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum InstTypeEnum {
    #[serde(rename="MARGIN")]
    Margin,
    #[serde(rename="SWAP")]
    Swap,
    #[serde(rename="FUTURES")]
    Futures,
    #[serde(rename="OPTION")]
    Option,
    #[serde(rename="ANY")]
    Any,
}

