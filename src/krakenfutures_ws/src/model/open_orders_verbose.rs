#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// OpenOrdersVerbose represents a union of types: OpenOrdersSnapshot, OpenOrdersResponse, OpenOrdersDeltaResponse, OpenOrdersCancelResponse, ErrorResponse
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(untagged)]
pub enum OpenOrdersVerbose {
    #[serde(rename="OpenOrdersSnapshot")]
    OpenOrdersSnapshot(OpenOrdersSnapshot),
    #[serde(rename="OpenOrdersResponse")]
    OpenOrdersResponse(OpenOrdersResponse),
    #[serde(rename="OpenOrdersDeltaResponse")]
    OpenOrdersDeltaResponse(OpenOrdersDeltaResponse),
    #[serde(rename="OpenOrdersCancelResponse")]
    OpenOrdersCancelResponse(OpenOrdersCancelResponse),
    #[serde(rename="ErrorResponse")]
    ErrorResponse(ErrorResponse),
}


