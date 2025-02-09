#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// FeedEnum represents a FeedEnum model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum FeedEnum {
    #[serde(rename="open_orders")]
    OpenOrders,
    #[serde(rename="open_orders_verbose")]
    OpenOrdersVerbose,
    #[serde(rename="fills")]
    Fills,
    #[serde(rename="balances")]
    Balances,
}

