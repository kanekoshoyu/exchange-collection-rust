#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// PosEnum represents a PosEnum model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum PosEnum {
    #[serde(rename="long")]
    Long,
    #[serde(rename="short")]
    Short,
    #[serde(rename="net")]
    Net,
}

