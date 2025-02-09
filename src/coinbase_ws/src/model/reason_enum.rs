#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// ReasonEnum represents a ReasonEnum model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum ReasonEnum {
    #[serde(rename="filled")]
    Filled,
    #[serde(rename="canceled")]
    Canceled,
}

