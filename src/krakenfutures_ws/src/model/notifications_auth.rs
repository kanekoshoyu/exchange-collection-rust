#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// NotificationsAuth represents a union of types: OpenOrdersResponse, NotificationsSnapshotResponse, ErrorResponse
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(untagged)]
pub enum NotificationsAuth {
    #[serde(rename="OpenOrdersResponse")]
    OpenOrdersResponse(OpenOrdersResponse),
    #[serde(rename="NotificationsSnapshotResponse")]
    NotificationsSnapshotResponse(NotificationsSnapshotResponse),
    #[serde(rename="ErrorResponse")]
    ErrorResponse(ErrorResponse),
}


