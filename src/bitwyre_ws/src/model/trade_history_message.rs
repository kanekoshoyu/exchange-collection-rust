#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// TradeHistoryMessage
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct TradeHistoryMessage(Vec<std::collections::HashMap<String, Trades>>);
