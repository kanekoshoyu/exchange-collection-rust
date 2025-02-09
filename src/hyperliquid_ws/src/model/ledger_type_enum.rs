#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// LedgerTypeEnum represents a LedgerTypeEnum model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum LedgerTypeEnum {
    #[serde(rename="liquidation")]
    Liquidation,
}

