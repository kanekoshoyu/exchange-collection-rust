#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// FillEnum represents a FillEnum model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum FillEnum {
    #[serde(rename="maker")]
    Maker,
    #[serde(rename="taker")]
    Taker,
    #[serde(rename="liquidation")]
    Liquidation,
    #[serde(rename="assignee")]
    Assignee,
    #[serde(rename="assignor")]
    Assignor,
    #[serde(rename="unwindBankrupt")]
    UnwindBankrupt,
    #[serde(rename="unwindCounterparty")]
    UnwindCounterparty,
    #[serde(rename="takerAfterEdit")]
    TakerAfterEdit,
}

