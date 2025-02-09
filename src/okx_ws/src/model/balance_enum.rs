#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// BalanceEnum represents a BalanceEnum model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum BalanceEnum {
    #[serde(rename="balance_and_position")]
    BalanceAndPosition,
}

