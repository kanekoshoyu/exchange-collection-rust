#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// OrderD represents a union of types: LimitMarketOrderInfo, StopLimitOrderInfo
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(untagged)]
pub enum OrderD {
    #[serde(rename="LimitMarketOrderInfo")]
    LimitMarketOrderInfo(LimitMarketOrderInfo),
    #[serde(rename="StopLimitOrderInfo")]
    StopLimitOrderInfo(StopLimitOrderInfo),
}


