#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// AccountLog represents a union of types: OpenOrdersResponse, LogsSnapshotResponse, LogsDeltaResponse, ErrorResponse
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(untagged)]
pub enum AccountLog {
    #[serde(rename="OpenOrdersResponse")]
    OpenOrdersResponse(OpenOrdersResponse),
    #[serde(rename="LogsSnapshotResponse")]
    LogsSnapshotResponse(LogsSnapshotResponse),
    #[serde(rename="LogsDeltaResponse")]
    LogsDeltaResponse(LogsDeltaResponse),
    #[serde(rename="ErrorResponse")]
    ErrorResponse(ErrorResponse),
}


