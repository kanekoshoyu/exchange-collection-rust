#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// AccountStatement represents a AccountStatement model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct AccountStatement {
    #[serde(rename="deposit")]
    pub deposit: std::collections::HashMap<String, std::collections::HashMap<String, TransactionHistory>>,
    #[serde(rename="withdrawal")]
    pub withdrawal: std::collections::HashMap<String, std::collections::HashMap<String, TransactionHistory>>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

