#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// EventEnum represents a EventEnum model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum EventEnum {
    #[serde(rename="snapshot")]
    Snapshot,
    #[serde(rename="delivered")]
    Delivered,
    #[serde(rename="exercised")]
    Exercised,
    #[serde(rename="transferred")]
    Transferred,
    #[serde(rename="filled")]
    Filled,
    #[serde(rename="liquidation")]
    Liquidation,
    #[serde(rename="claw_back")]
    ClawBack,
    #[serde(rename="adl")]
    Adl,
    #[serde(rename="funding_fee")]
    FundingFee,
    #[serde(rename="adjust_margin")]
    AdjustMargin,
    #[serde(rename="set_leverage")]
    SetLeverage,
    #[serde(rename="interest_deduction")]
    InterestDeduction,
}

