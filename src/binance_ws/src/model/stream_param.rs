#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// StreamParam represents a union of types: String, bool
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(untagged)]
pub enum StreamParam {
    #[serde(rename="StringResponse")]
    StringResponse(String),
    #[serde(rename="BoolResponse")]
    BoolResponse(bool),
}


