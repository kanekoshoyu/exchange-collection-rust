#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// Fills represents a union of types: FillsResponse, FillsSnapshotResponse, FillsSnapshotResponse, OpenOrdersCancelResponse, ErrorResponse
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(untagged)]
pub enum Fills {
    #[serde(rename="FillsResponse")]
    FillsResponse(FillsResponse),
    #[serde(rename="FillsSnapshotResponse")]
    FillsSnapshotResponse(FillsSnapshotResponse),
    #[serde(rename="FillsSnapshotResponse")]
    FillsSnapshotResponse(FillsSnapshotResponse),
    #[serde(rename="OpenOrdersCancelResponse")]
    OpenOrdersCancelResponse(OpenOrdersCancelResponse),
    #[serde(rename="ErrorResponse")]
    ErrorResponse(ErrorResponse),
}


