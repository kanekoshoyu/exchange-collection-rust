#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// UnitEnum represents a UnitEnum model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum UnitEnum {
    #[serde(rename="percent")]
    Percent,
}

