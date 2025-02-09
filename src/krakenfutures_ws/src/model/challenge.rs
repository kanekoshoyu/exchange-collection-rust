#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// Challenge represents a union of types: ChallengeResponse, ErrorResponse
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(untagged)]
pub enum Challenge {
    #[serde(rename="ChallengeResponse")]
    ChallengeResponse(ChallengeResponse),
    #[serde(rename="ErrorResponse")]
    ErrorResponse(ErrorResponse),
}


