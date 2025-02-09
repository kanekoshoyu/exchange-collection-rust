#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// DirectionEnum represents a DirectionEnum model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum DirectionEnum {
    #[serde(rename="1")]
    Number1,
    #[serde(rename="-1")]
    Minus_1,
}

