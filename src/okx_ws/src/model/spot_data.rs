#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// SpotData represents a SpotData model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct SpotData {
    #[serde(rename="algoClOrdId", skip_serializing_if = "Option::is_none")]
    pub algo_cl_ord_id: Option<String>,
    #[serde(rename="algoId", skip_serializing_if = "Option::is_none")]
    pub algo_id: Option<String>,
    #[serde(rename="activeOrdNum", skip_serializing_if = "Option::is_none")]
    pub active_ord_num: Option<String>,
    #[serde(rename="algoOrdType", skip_serializing_if = "Option::is_none")]
    pub algo_ord_type: Option<String>,
    #[serde(rename="annualizedRate", skip_serializing_if = "Option::is_none")]
    pub annualized_rate: Option<String>,
    #[serde(rename="arbitrageNum", skip_serializing_if = "Option::is_none")]
    pub arbitrage_num: Option<String>,
    #[serde(rename="baseSz", skip_serializing_if = "Option::is_none")]
    pub base_sz: Option<String>,
    #[serde(rename="cTime", skip_serializing_if = "Option::is_none")]
    pub c_time: Option<String>,
    #[serde(rename="cancelType", skip_serializing_if = "Option::is_none")]
    pub cancel_type: Option<String>,
    #[serde(rename="curBaseSz", skip_serializing_if = "Option::is_none")]
    pub cur_base_sz: Option<String>,
    #[serde(rename="curQuoteSz", skip_serializing_if = "Option::is_none")]
    pub cur_quote_sz: Option<String>,
    #[serde(rename="floatProfit", skip_serializing_if = "Option::is_none")]
    pub float_profit: Option<String>,
    #[serde(rename="gridNum", skip_serializing_if = "Option::is_none")]
    pub grid_num: Option<String>,
    #[serde(rename="gridProfit", skip_serializing_if = "Option::is_none")]
    pub grid_profit: Option<String>,
    #[serde(rename="instId", skip_serializing_if = "Option::is_none")]
    pub inst_id: Option<String>,
    #[serde(rename="instType", skip_serializing_if = "Option::is_none")]
    pub inst_type: Option<String>,
    #[serde(rename="investment", skip_serializing_if = "Option::is_none")]
    pub investment: Option<String>,
    #[serde(rename="maxPx", skip_serializing_if = "Option::is_none")]
    pub max_px: Option<String>,
    #[serde(rename="minPx", skip_serializing_if = "Option::is_none")]
    pub min_px: Option<String>,
    #[serde(rename="pTime", skip_serializing_if = "Option::is_none")]
    pub p_time: Option<String>,
    #[serde(rename="perMaxProfitRate", skip_serializing_if = "Option::is_none")]
    pub per_max_profit_rate: Option<String>,
    #[serde(rename="perMinProfitRate", skip_serializing_if = "Option::is_none")]
    pub per_min_profit_rate: Option<String>,
    #[serde(rename="pnlRatio", skip_serializing_if = "Option::is_none")]
    pub pnl_ratio: Option<String>,
    #[serde(rename="profit", skip_serializing_if = "Option::is_none")]
    pub profit: Option<String>,
    #[serde(rename="quoteSz", skip_serializing_if = "Option::is_none")]
    pub quote_sz: Option<String>,
    #[serde(rename="rebateTrans", skip_serializing_if = "Option::is_none")]
    pub rebate_trans: Option<Vec<RebateData>>,
    #[serde(rename="runPx", skip_serializing_if = "Option::is_none")]
    pub run_px: Option<String>,
    #[serde(rename="runType", skip_serializing_if = "Option::is_none")]
    pub run_type: Option<String>,
    #[serde(rename="triggerParams", skip_serializing_if = "Option::is_none")]
    pub trigger_params: Option<Vec<TriggerData>>,
    #[serde(rename="singleAmt", skip_serializing_if = "Option::is_none")]
    pub single_amt: Option<String>,
    #[serde(rename="slTriggerPx", skip_serializing_if = "Option::is_none")]
    pub sl_trigger_px: Option<String>,
    #[serde(rename="state", skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename="stopResult", skip_serializing_if = "Option::is_none")]
    pub stop_result: Option<String>,
    #[serde(rename="stopType", skip_serializing_if = "Option::is_none")]
    pub stop_type: Option<String>,
    #[serde(rename="tag", skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    #[serde(rename="totalAnnualizedRate", skip_serializing_if = "Option::is_none")]
    pub total_annualized_rate: Option<String>,
    #[serde(rename="totalPnl", skip_serializing_if = "Option::is_none")]
    pub total_pnl: Option<String>,
    #[serde(rename="tpTriggerPx", skip_serializing_if = "Option::is_none")]
    pub tp_trigger_px: Option<String>,
    #[serde(rename="tradeNum", skip_serializing_if = "Option::is_none")]
    pub trade_num: Option<String>,
    #[serde(rename="uTime", skip_serializing_if = "Option::is_none")]
    pub u_time: Option<String>,
    #[serde(rename="profitSharingRatio", skip_serializing_if = "Option::is_none")]
    pub profit_sharing_ratio: Option<String>,
    #[serde(rename="copyType", skip_serializing_if = "Option::is_none")]
    pub copy_type: Option<String>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

