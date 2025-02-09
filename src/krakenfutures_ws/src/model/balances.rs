#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// Balances represents a union of types: OpenOrdersResponse, FillsSnapshotResponse, AnonymousSchema46, ErrorResponse
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(untagged)]
pub enum Balances {
    #[serde(rename="OpenOrdersResponse")]
    OpenOrdersResponse(OpenOrdersResponse),
    #[serde(rename="FillsSnapshotResponse")]
    FillsSnapshotResponse(FillsSnapshotResponse),
    #[serde(rename="AnonymousSchema46")]
    AnonymousSchema46(AnonymousSchema46),
    #[serde(rename="ErrorResponse")]
    ErrorResponse(ErrorResponse),
}


