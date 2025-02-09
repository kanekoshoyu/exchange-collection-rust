#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// Book represents a union of types: TickerResponse, BookSnapshotResponse, BookDeltaResponse, ErrorResponse
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(untagged)]
pub enum Book {
    #[serde(rename="TickerResponse")]
    TickerResponse(TickerResponse),
    #[serde(rename="BookSnapshotResponse")]
    BookSnapshotResponse(BookSnapshotResponse),
    #[serde(rename="BookDeltaResponse")]
    BookDeltaResponse(BookDeltaResponse),
    #[serde(rename="ErrorResponse")]
    ErrorResponse(ErrorResponse),
}


