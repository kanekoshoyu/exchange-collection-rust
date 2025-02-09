#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// WithdrawPushResponse
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct WithdrawPushResponse(serde_json::Value);
