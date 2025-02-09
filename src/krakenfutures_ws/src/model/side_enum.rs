#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// SideEnum represents a SideEnum model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum SideEnum {
    #[serde(rename="buy")]
    Buy,
    #[serde(rename="sell")]
    Sell,
}

