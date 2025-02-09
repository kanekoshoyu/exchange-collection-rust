#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// Args represents a union of types: GeneralRequest, LoginRequest
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(untagged)]
pub enum Args {
    #[serde(rename="GeneralRequest")]
    GeneralRequest(GeneralRequest),
    #[serde(rename="LoginRequest")]
    LoginRequest(LoginRequest),
}


