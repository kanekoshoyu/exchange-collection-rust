#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// TradeData
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct TradeData(Vec<Vec<String>>);
