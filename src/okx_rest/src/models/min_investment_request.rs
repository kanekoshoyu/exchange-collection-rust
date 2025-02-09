/*
 * OKX API
 *
 * Welcome to OKX Developer document!   excluded below endpoints as they are if you need them please add and commit to https://github.com/kanekoshoyu/exchange-collection): - Trading Account (this might become needed, will add when we need it) - Block Trading - Financial Producer - Affiliate - Status - Announcement 
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct MinInvestmentRequest {
    /// Instrument ID, e.g., BTC-USDT-SWAP
    #[serde(rename = "instId", skip_serializing_if = "Option::is_none")]
    pub inst_id: Option<String>,
    /// Algo order type (Spot grid or Contract grid)
    #[serde(rename = "algoOrdType", skip_serializing_if = "Option::is_none")]
    pub algo_ord_type: Option<AlgoOrdType>,
    /// Upper price of price range
    #[serde(rename = "maxPx", skip_serializing_if = "Option::is_none")]
    pub max_px: Option<String>,
    /// Lower price of price range
    #[serde(rename = "minPx", skip_serializing_if = "Option::is_none")]
    pub min_px: Option<String>,
    /// Grid quantity
    #[serde(rename = "gridNum", skip_serializing_if = "Option::is_none")]
    pub grid_num: Option<String>,
    /// Grid type (1: Arithmetic, 2: Geometric)
    #[serde(rename = "runType", skip_serializing_if = "Option::is_none")]
    pub run_type: Option<RunType>,
    /// Contract grid type (only applicable to contract grid)
    #[serde(rename = "direction", skip_serializing_if = "Option::is_none")]
    pub direction: Option<Direction>,
    /// Leverage (only applicable to contract grid)
    #[serde(rename = "lever", skip_serializing_if = "Option::is_none")]
    pub lever: Option<String>,
    /// Whether or not to open a position when the strategy activates. Default is false. Neutral contract grid should omit the parameter. (Only applicable to contract grid)
    #[serde(rename = "basePos", skip_serializing_if = "Option::is_none")]
    pub base_pos: Option<bool>,
    /// Investment type. Only applicable to grid.
    #[serde(rename = "investmentType", skip_serializing_if = "Option::is_none")]
    pub investment_type: Option<InvestmentType>,
    /// Trigger strategy.
    #[serde(rename = "triggerStrategy", skip_serializing_if = "Option::is_none")]
    pub trigger_strategy: Option<TriggerStrategy>,
    #[serde(rename = "investmentData", skip_serializing_if = "Option::is_none")]
    pub investment_data: Option<Vec<models::MinInvestmentRequestInvestmentDataInner>>,
}

impl MinInvestmentRequest {
    pub fn new() -> MinInvestmentRequest {
        MinInvestmentRequest {
            inst_id: None,
            algo_ord_type: None,
            max_px: None,
            min_px: None,
            grid_num: None,
            run_type: None,
            direction: None,
            lever: None,
            base_pos: None,
            investment_type: None,
            trigger_strategy: None,
            investment_data: None,
        }
    }
}
/// Algo order type (Spot grid or Contract grid)
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AlgoOrdType {
    #[serde(rename = "grid")]
    Grid,
    #[serde(rename = "contract_grid")]
    ContractGrid,
}

impl Default for AlgoOrdType {
    fn default() -> AlgoOrdType {
        Self::Grid
    }
}
/// Grid type (1: Arithmetic, 2: Geometric)
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RunType {
    #[serde(rename = "1")]
    Variant1,
    #[serde(rename = "2")]
    Variant2,
}

impl Default for RunType {
    fn default() -> RunType {
        Self::Variant1
    }
}
/// Contract grid type (only applicable to contract grid)
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Direction {
    #[serde(rename = "long")]
    Long,
    #[serde(rename = "short")]
    Short,
    #[serde(rename = "neutral")]
    Neutral,
}

impl Default for Direction {
    fn default() -> Direction {
        Self::Long
    }
}
/// Investment type. Only applicable to grid.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum InvestmentType {
    #[serde(rename = "quote")]
    Quote,
    #[serde(rename = "base")]
    Base,
    #[serde(rename = "dual")]
    Dual,
}

impl Default for InvestmentType {
    fn default() -> InvestmentType {
        Self::Quote
    }
}
/// Trigger strategy.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TriggerStrategy {
    #[serde(rename = "instant")]
    Instant,
    #[serde(rename = "price")]
    Price,
    #[serde(rename = "rsi")]
    Rsi,
}

impl Default for TriggerStrategy {
    fn default() -> TriggerStrategy {
        Self::Instant
    }
}

