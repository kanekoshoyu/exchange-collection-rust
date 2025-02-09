#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// OrderEnum represents a OrderEnum model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum OrderEnum {
    #[serde(rename="NEW")]
    New,
    #[serde(rename="PARTIALLY_FILLED")]
    PartiallyFilled,
    #[serde(rename="FILLED")]
    Filled,
    #[serde(rename="PARTIALLY_CANCELED")]
    PartiallyCanceled,
    #[serde(rename="CANCELED")]
    Canceled,
    #[serde(rename="REJECTED")]
    Rejected,
}

