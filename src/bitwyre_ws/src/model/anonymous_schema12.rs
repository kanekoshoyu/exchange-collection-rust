#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// AnonymousSchema12
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct AnonymousSchema12(serde_json::Value);
