#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// ChannelEnum represents a ChannelEnum model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum ChannelEnum {
    #[serde(rename="ticker")]
    Ticker,
    #[serde(rename="candle1m")]
    Candle1m,
    #[serde(rename="candle5m")]
    Candle5m,
    #[serde(rename="candle15m")]
    Candle15m,
    #[serde(rename="candle30m")]
    Candle30m,
    #[serde(rename="candle1H")]
    Candle1H,
    #[serde(rename="candle4H")]
    Candle4H,
    #[serde(rename="candle6H")]
    Candle6H,
    #[serde(rename="candle12H")]
    Candle12H,
    #[serde(rename="candle1D")]
    Candle1D,
    #[serde(rename="candle3D")]
    Candle3D,
    #[serde(rename="candle1W")]
    Candle1W,
    #[serde(rename="candle1M")]
    Candle1M,
    #[serde(rename="candle6Hutc")]
    Candle6Hutc,
    #[serde(rename="candle12Hutc")]
    Candle12Hutc,
    #[serde(rename="candle1Dutc")]
    Candle1Dutc,
    #[serde(rename="candle3Dutc")]
    Candle3Dutc,
    #[serde(rename="candle1Wutc")]
    Candle1Wutc,
    #[serde(rename="candle1Mutc")]
    Candle1Mutc,
    #[serde(rename="trade")]
    Trade,
    #[serde(rename="book")]
    Book,
    #[serde(rename="book1")]
    Book1,
    #[serde(rename="book5")]
    Book5,
    #[serde(rename="book15")]
    Book15,
    #[serde(rename="fill")]
    Fill,
    #[serde(rename="orders")]
    Orders,
    #[serde(rename="orders-algo")]
    OrdersMinusAlgo,
    #[serde(rename="account")]
    Account,
    #[serde(rename="positions")]
    Positions,
}

