#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// ExpiryEnum represents a ExpiryEnum model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum ExpiryEnum {
    #[serde(rename="UNKNOWN_CONTRACT_EXPIRY_TYPE")]
    UnknownContractExpiryType,
    #[serde(rename="EXPIRY")]
    Expiry,
    #[serde(rename="PERPECTUAL")]
    Perpectual,
}

