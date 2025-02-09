#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// AccountEnum represents a AccountEnum model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum AccountEnum {
    #[serde(rename="subAccountTransfer")]
    SubAccountTransfer,
}

