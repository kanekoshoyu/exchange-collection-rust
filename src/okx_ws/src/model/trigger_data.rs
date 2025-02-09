#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// TriggerData represents a TriggerData model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct TriggerData {
    #[serde(rename="triggerAction", skip_serializing_if = "Option::is_none")]
    pub trigger_action: Option<String>,
    #[serde(rename="triggerStrategy", skip_serializing_if = "Option::is_none")]
    pub trigger_strategy: Option<String>,
    #[serde(rename="delaySeconds", skip_serializing_if = "Option::is_none")]
    pub delay_seconds: Option<String>,
    #[serde(rename="triggerTime", skip_serializing_if = "Option::is_none")]
    pub trigger_time: Option<String>,
    #[serde(rename="triggerType", skip_serializing_if = "Option::is_none")]
    pub trigger_type: Option<String>,
    #[serde(rename="timeframe", skip_serializing_if = "Option::is_none")]
    pub timeframe: Option<String>,
    #[serde(rename="thold", skip_serializing_if = "Option::is_none")]
    pub thold: Option<i32>,
    #[serde(rename="triggerCond", skip_serializing_if = "Option::is_none")]
    pub trigger_cond: Option<String>,
    #[serde(rename="timePeriod", skip_serializing_if = "Option::is_none")]
    pub time_period: Option<String>,
    #[serde(rename="triggerPx", skip_serializing_if = "Option::is_none")]
    pub trigger_px: Option<String>,
    #[serde(rename="stopType", skip_serializing_if = "Option::is_none")]
    pub stop_type: Option<String>,
    #[serde(rename="maxPx", skip_serializing_if = "Option::is_none")]
    pub max_px: Option<String>,
    #[serde(rename="minPx", skip_serializing_if = "Option::is_none")]
    pub min_px: Option<String>,
    #[serde(rename="gridNum", skip_serializing_if = "Option::is_none")]
    pub grid_num: Option<String>,
    #[serde(rename="runType", skip_serializing_if = "Option::is_none")]
    pub run_type: Option<String>,
    #[serde(rename="tpTriggerPx", skip_serializing_if = "Option::is_none")]
    pub tp_trigger_px: Option<String>,
    #[serde(rename="slTriggerPx", skip_serializing_if = "Option::is_none")]
    pub sl_trigger_px: Option<String>,
    #[serde(rename="tradeNum", skip_serializing_if = "Option::is_none")]
    pub trade_num: Option<String>,
    #[serde(rename="arbitrageNum", skip_serializing_if = "Option::is_none")]
    pub arbitrage_num: Option<String>,
    #[serde(rename="singleAmt", skip_serializing_if = "Option::is_none")]
    pub single_amt: Option<String>,
    #[serde(rename="perMinProfitRate", skip_serializing_if = "Option::is_none")]
    pub per_min_profit_rate: Option<String>,
    #[serde(rename="perMaxProfitRate", skip_serializing_if = "Option::is_none")]
    pub per_max_profit_rate: Option<String>,
    #[serde(rename="runPx", skip_serializing_if = "Option::is_none")]
    pub run_px: Option<String>,
    #[serde(rename="totalPnl", skip_serializing_if = "Option::is_none")]
    pub total_pnl: Option<String>,
    #[serde(rename="pnlRatio", skip_serializing_if = "Option::is_none")]
    pub pnl_ratio: Option<String>,
    #[serde(rename="investment", skip_serializing_if = "Option::is_none")]
    pub investment: Option<String>,
    #[serde(rename="gridProfit", skip_serializing_if = "Option::is_none")]
    pub grid_profit: Option<String>,
    #[serde(rename="floatProfit", skip_serializing_if = "Option::is_none")]
    pub float_profit: Option<String>,
    #[serde(rename="totalAnnualizedRate", skip_serializing_if = "Option::is_none")]
    pub total_annualized_rate: Option<String>,
    #[serde(rename="annualizedRate", skip_serializing_if = "Option::is_none")]
    pub annualized_rate: Option<String>,
    #[serde(rename="cancelType", skip_serializing_if = "Option::is_none")]
    pub cancel_type: Option<String>,
    #[serde(rename="direction", skip_serializing_if = "Option::is_none")]
    pub direction: Option<String>,
    #[serde(rename="basePos", skip_serializing_if = "Option::is_none")]
    pub base_pos: Option<bool>,
    #[serde(rename="sz", skip_serializing_if = "Option::is_none")]
    pub sz: Option<String>,
    #[serde(rename="lever", skip_serializing_if = "Option::is_none")]
    pub lever: Option<String>,
    #[serde(rename="actualLever", skip_serializing_if = "Option::is_none")]
    pub actual_lever: Option<String>,
    #[serde(rename="liqPx", skip_serializing_if = "Option::is_none")]
    pub liq_px: Option<String>,
    #[serde(rename="ordFrozen", skip_serializing_if = "Option::is_none")]
    pub ord_frozen: Option<String>,
    #[serde(rename="availEq", skip_serializing_if = "Option::is_none")]
    pub avail_eq: Option<String>,
    #[serde(rename="eq", skip_serializing_if = "Option::is_none")]
    pub eq: Option<String>,
    #[serde(rename="activeOrdNum", skip_serializing_if = "Option::is_none")]
    pub active_ord_num: Option<String>,
    #[serde(rename="tag", skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    #[serde(rename="profitSharingRatio", skip_serializing_if = "Option::is_none")]
    pub profit_sharing_ratio: Option<String>,
    #[serde(rename="copyType", skip_serializing_if = "Option::is_none")]
    pub copy_type: Option<String>,
    #[serde(rename="tpRatio", skip_serializing_if = "Option::is_none")]
    pub tp_ratio: Option<String>,
    #[serde(rename="slRatio", skip_serializing_if = "Option::is_none")]
    pub sl_ratio: Option<String>,
    #[serde(rename="fee", skip_serializing_if = "Option::is_none")]
    pub fee: Option<String>,
    #[serde(rename="fundingFee", skip_serializing_if = "Option::is_none")]
    pub funding_fee: Option<String>,
    #[serde(rename="pTime", skip_serializing_if = "Option::is_none")]
    pub p_time: Option<String>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

