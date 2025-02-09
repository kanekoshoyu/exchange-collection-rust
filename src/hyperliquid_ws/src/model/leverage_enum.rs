#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// LeverageEnum represents a LeverageEnum model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum LeverageEnum {
    #[serde(rename="Cross")]
    Cross,
    #[serde(rename="Isolated")]
    Isolated,
}

