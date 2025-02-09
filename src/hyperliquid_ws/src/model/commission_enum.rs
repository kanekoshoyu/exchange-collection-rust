#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// CommissionEnum represents a CommissionEnum model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum CommissionEnum {
    #[serde(rename="vaultLeaderCommission")]
    VaultLeaderCommission,
}

