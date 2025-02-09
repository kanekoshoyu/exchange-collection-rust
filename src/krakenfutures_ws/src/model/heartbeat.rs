#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// Heartbeat represents a union of types: HeartbeatRequest, HeartbeaSnapshotRequest, ErrorResponse
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(untagged)]
pub enum Heartbeat {
    #[serde(rename="HeartbeatRequest")]
    HeartbeatRequest(HeartbeatRequest),
    #[serde(rename="HeartbeaSnapshotRequest")]
    HeartbeaSnapshotRequest(HeartbeaSnapshotRequest),
    #[serde(rename="ErrorResponse")]
    ErrorResponse(ErrorResponse),
}


