#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// VaultEnum represents a VaultEnum model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum VaultEnum {
    #[serde(rename="vaultCreate")]
    VaultCreate,
    #[serde(rename="vaultDeposit")]
    VaultDeposit,
    #[serde(rename="vaultDistribution")]
    VaultDistribution,
}

