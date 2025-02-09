#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// OrderbookData
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct OrderbookData(Vec<Union>);
