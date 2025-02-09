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
pub struct TpslSignal {
    /// The unique identifier for the algorithm.
    #[serde(rename = "algoId")]
    pub algo_id: String,
    /// The parameters for setting exit strategies.
    #[serde(rename = "exitSettingParam")]
    pub exit_setting_param: String,
    /// The type of method used to trigger the take-profit and stop-loss prices. Possible values: - \"pnl\": Based on the estimated profit and loss percentage from the entry point. - \"price\": Based on price increase or decrease from the crypto’s entry price. 
    #[serde(rename = "tpSlType")]
    pub tp_sl_type: String,
    /// The take-profit percentage. This value determines when to exit based on the profit target. 
    #[serde(rename = "tpPct", skip_serializing_if = "Option::is_none")]
    pub tp_pct: Option<String>,
    /// The stop-loss percentage. This value determines when to exit based on the loss threshold. 
    #[serde(rename = "slPct", skip_serializing_if = "Option::is_none")]
    pub sl_pct: Option<String>,
}

impl TpslSignal {
    pub fn new(algo_id: String, exit_setting_param: String, tp_sl_type: String) -> TpslSignal {
        TpslSignal {
            algo_id,
            exit_setting_param,
            tp_sl_type,
            tp_pct: None,
            sl_pct: None,
        }
    }
}

