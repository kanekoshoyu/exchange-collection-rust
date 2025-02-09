#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// L2Enum represents a L2Enum model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum L2Enum {
    #[serde(rename="l2Book")]
    L2Book,
}

