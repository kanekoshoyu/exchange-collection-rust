#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// OrderEnum represents a OrderEnum model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum OrderEnum {
    #[serde(rename="market")]
    Market,
    #[serde(rename="limit")]
    Limit,
    #[serde(rename="post_only")]
    PostOnly,
    #[serde(rename="fok")]
    Fok,
    #[serde(rename="ioc")]
    Ioc,
    #[serde(rename="optimal_limit_ioc")]
    OptimalLimitIoc,
    #[serde(rename="mmp")]
    Mmp,
    #[serde(rename="mmp_and_post_only")]
    MmpAndPostOnly,
    #[serde(rename="op_fok")]
    OpFok,
}

