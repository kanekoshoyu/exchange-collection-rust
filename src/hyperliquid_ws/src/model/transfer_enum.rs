#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// TransferEnum represents a TransferEnum model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum TransferEnum {
    #[serde(rename="internalTransfer")]
    InternalTransfer,
}

