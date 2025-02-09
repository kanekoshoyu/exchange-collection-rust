#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// Payload represents a union of types: GeneralResponse, PushResponse
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(untagged)]
pub enum Payload {
    #[serde(rename="GeneralResponse")]
    GeneralResponse(GeneralResponse),
    #[serde(rename="PushResponse")]
    PushResponse(PushResponse),
}


