#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// OpenPositions represents a union of types: OpenOrdersResponse, PositionsSnapshotResponse, ErrorResponse
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(untagged)]
pub enum OpenPositions {
    #[serde(rename="OpenOrdersResponse")]
    OpenOrdersResponse(OpenOrdersResponse),
    #[serde(rename="PositionsSnapshotResponse")]
    PositionsSnapshotResponse(PositionsSnapshotResponse),
    #[serde(rename="ErrorResponse")]
    ErrorResponse(ErrorResponse),
}


