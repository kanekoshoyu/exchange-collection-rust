#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// TypeEnum represents a TypeEnum model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum TypeEnum {
    #[serde(rename="buy")]
    Buy,
    #[serde(rename="sell")]
    Sell,
}

