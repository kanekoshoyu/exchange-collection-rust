/*
 * Kraken API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SendOrderResponse {
    /// Server time in Coordinated Universal Time (UTC).
    #[serde(rename = "serverTime", skip_serializing_if = "Option::is_none")]
    pub server_time: Option<String>,
    /// Indicates the success of the request.
    #[serde(rename = "result", skip_serializing_if = "Option::is_none")]
    pub result: Option<Result>,
    /// The unique identifier of the order.
    #[serde(rename = "order_id", skip_serializing_if = "Option::is_none")]
    pub order_id: Option<uuid::Uuid>,
    /// The date and time the order was received.
    #[serde(rename = "receivedTime", skip_serializing_if = "Option::is_none")]
    pub received_time: Option<String>,
    #[serde(rename = "orderEvents", skip_serializing_if = "Option::is_none")]
    pub order_events: Option<Vec<models::PlaceEvent>>,
    /// The status of the order.
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    #[serde(rename = "sendStatus", skip_serializing_if = "Option::is_none")]
    pub send_status: Option<Box<models::SendOrderResponseSendStatus>>,
}

impl SendOrderResponse {
    pub fn new() -> SendOrderResponse {
        SendOrderResponse {
            server_time: None,
            result: None,
            order_id: None,
            received_time: None,
            order_events: None,
            status: None,
            send_status: None,
        }
    }
}
/// Indicates the success of the request.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Result {
    #[serde(rename = "success")]
    Success,
}

impl Default for Result {
    fn default() -> Result {
        Self::Success
    }
}
/// The status of the order.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "placed")]
    Placed,
    #[serde(rename = "partiallyFilled")]
    PartiallyFilled,
    #[serde(rename = "filled")]
    Filled,
    #[serde(rename = "cancelled")]
    Cancelled,
    #[serde(rename = "edited")]
    Edited,
    #[serde(rename = "marketSuspended")]
    MarketSuspended,
    #[serde(rename = "marketInactive")]
    MarketInactive,
    #[serde(rename = "invalidPrice")]
    InvalidPrice,
    #[serde(rename = "invalidSize")]
    InvalidSize,
    #[serde(rename = "tooManySmallOrders")]
    TooManySmallOrders,
    #[serde(rename = "insufficientAvailableFunds")]
    InsufficientAvailableFunds,
    #[serde(rename = "wouldCauseLiquidation")]
    WouldCauseLiquidation,
    #[serde(rename = "clientOrderIdAlreadyExist")]
    ClientOrderIdAlreadyExist,
    #[serde(rename = "clientOrderIdTooBig")]
    ClientOrderIdTooBig,
    #[serde(rename = "maxPositionViolation")]
    MaxPositionViolation,
    #[serde(rename = "outsidePriceCollar")]
    OutsidePriceCollar,
    #[serde(rename = "wouldIncreasePriceDislocation")]
    WouldIncreasePriceDislocation,
    #[serde(rename = "notFound")]
    NotFound,
    #[serde(rename = "orderForEditNotAStop")]
    OrderForEditNotAStop,
    #[serde(rename = "orderForEditNotFound")]
    OrderForEditNotFound,
    #[serde(rename = "postWouldExecute")]
    PostWouldExecute,
    #[serde(rename = "iocWouldNotExecute")]
    IocWouldNotExecute,
    #[serde(rename = "selfFill")]
    SelfFill,
    #[serde(rename = "wouldNotReducePosition")]
    WouldNotReducePosition,
    #[serde(rename = "marketIsPostOnly")]
    MarketIsPostOnly,
    #[serde(rename = "tooManyOrders")]
    TooManyOrders,
    #[serde(rename = "fixedLeverageTooHigh")]
    FixedLeverageTooHigh,
    #[serde(rename = "clientOrderIdInvalid")]
    ClientOrderIdInvalid,
    #[serde(rename = "cannotEditTriggerPriceOfTrailingStop")]
    CannotEditTriggerPriceOfTrailingStop,
    #[serde(rename = "cannotEditLimitPriceOfTrailingStop")]
    CannotEditLimitPriceOfTrailingStop,
    #[serde(rename = "wouldProcessAfterSpecifiedTime")]
    WouldProcessAfterSpecifiedTime,
}

impl Default for Status {
    fn default() -> Status {
        Self::Placed
    }
}

