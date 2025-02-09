#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// DepositPushResponse
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct DepositPushResponse(serde_json::Value);
