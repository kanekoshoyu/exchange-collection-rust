#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// VaultWithdrawEnum represents a VaultWithdrawEnum model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum VaultWithdrawEnum {
    #[serde(rename="vaultWithdraw")]
    VaultWithdraw,
}

