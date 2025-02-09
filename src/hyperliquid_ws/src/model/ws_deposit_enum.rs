#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// WsDepositEnum represents a WsDepositEnum model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum WsDepositEnum {
    #[serde(rename="deposit")]
    Deposit,
}

