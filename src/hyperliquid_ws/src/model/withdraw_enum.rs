#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// WithdrawEnum represents a WithdrawEnum model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum WithdrawEnum {
    #[serde(rename="withdraw")]
    Withdraw,
}

