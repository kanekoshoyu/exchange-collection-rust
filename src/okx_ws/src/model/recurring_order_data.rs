#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// RecurringOrderData represents a RecurringOrderData model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct RecurringOrderData {
    #[serde(rename="algoId", skip_serializing_if = "Option::is_none")]
    pub algo_id: Option<String>,
    #[serde(rename="algoClOrdId", skip_serializing_if = "Option::is_none")]
    pub algo_cl_ord_id: Option<String>,
    #[serde(rename="instType", skip_serializing_if = "Option::is_none")]
    pub inst_type: Option<String>,
    #[serde(rename="cTime", skip_serializing_if = "Option::is_none")]
    pub c_time: Option<String>,
    #[serde(rename="uTime", skip_serializing_if = "Option::is_none")]
    pub u_time: Option<String>,
    #[serde(rename="algoOrdType", skip_serializing_if = "Option::is_none")]
    pub algo_ord_type: Option<String>,
    #[serde(rename="state", skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename="stgyName", skip_serializing_if = "Option::is_none")]
    pub stgy_name: Option<String>,
    #[serde(rename="recurringList", skip_serializing_if = "Option::is_none")]
    pub recurring_list: Option<Vec<RecurringBuyDetails>>,
    #[serde(rename="period", skip_serializing_if = "Option::is_none")]
    pub period: Option<String>,
    #[serde(rename="recurringDay", skip_serializing_if = "Option::is_none")]
    pub recurring_day: Option<String>,
    #[serde(rename="recurringHour", skip_serializing_if = "Option::is_none")]
    pub recurring_hour: Option<String>,
    #[serde(rename="recurringTime", skip_serializing_if = "Option::is_none")]
    pub recurring_time: Option<String>,
    #[serde(rename="timeZone", skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
    #[serde(rename="amt", skip_serializing_if = "Option::is_none")]
    pub amt: Option<String>,
    #[serde(rename="investmentAmt", skip_serializing_if = "Option::is_none")]
    pub investment_amt: Option<String>,
    #[serde(rename="investmentCcy", skip_serializing_if = "Option::is_none")]
    pub investment_ccy: Option<String>,
    #[serde(rename="nextInvestTime", skip_serializing_if = "Option::is_none")]
    pub next_invest_time: Option<String>,
    #[serde(rename="totalPnl", skip_serializing_if = "Option::is_none")]
    pub total_pnl: Option<String>,
    #[serde(rename="totalAnnRate", skip_serializing_if = "Option::is_none")]
    pub total_ann_rate: Option<String>,
    #[serde(rename="pnlRatio", skip_serializing_if = "Option::is_none")]
    pub pnl_ratio: Option<String>,
    #[serde(rename="mktCap", skip_serializing_if = "Option::is_none")]
    pub mkt_cap: Option<String>,
    #[serde(rename="cycles", skip_serializing_if = "Option::is_none")]
    pub cycles: Option<String>,
    #[serde(rename="tag", skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    #[serde(rename="pTime", skip_serializing_if = "Option::is_none")]
    pub p_time: Option<String>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

