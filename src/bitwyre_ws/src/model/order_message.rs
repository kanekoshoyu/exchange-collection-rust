#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// OrderMessage represents a OrderMessage model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct OrderMessage {
    #[serde(rename="OrderID")]
    pub order_id: String,
    #[serde(rename="ClOrdID")]
    pub cl_ord_id: String,
    #[serde(rename="OrigClID")]
    pub orig_cl_id: String,
    #[serde(rename="OrdStatusReqID")]
    pub ord_status_req_id: String,
    #[serde(rename="ExecID")]
    pub exec_id: String,
    #[serde(rename="ExecType")]
    pub exec_type: Box<ExecEnum>,
    #[serde(rename="OrdStatus")]
    pub ord_status: Box<OrderEnum>,
    #[serde(rename="OrdRejReason")]
    pub ord_rej_reason: Box<RejectionEnum>,
    #[serde(rename="Account")]
    pub account: String,
    #[serde(rename="Instrument")]
    pub instrument: String,
    #[serde(rename="Side")]
    pub side: i32,
    #[serde(rename="TransactTime")]
    pub transact_time: i64,
    #[serde(rename="OrderQty")]
    pub order_qty: String,
    #[serde(rename="Timestamp")]
    pub timestamp: i64,
    #[serde(rename="OrdType")]
    pub ord_type: i32,
    #[serde(rename="Price")]
    pub price: String,
    #[serde(rename="StopPx")]
    pub stop_px: String,
    #[serde(rename="TimeInForce")]
    pub time_in_force: Box<TimeEnum>,
    #[serde(rename="ExpireTime")]
    pub expire_time: i64,
    #[serde(rename="CancelOnDisconnect")]
    pub cancel_on_disconnect: bool,
    #[serde(rename="LastQty")]
    pub last_qty: String,
    #[serde(rename="LastPx")]
    pub last_px: String,
    #[serde(rename="LastLiquidityInd")]
    pub last_liquidity_ind: String,
    #[serde(rename="LeavesQty")]
    pub leaves_qty: String,
    #[serde(rename="CumQty")]
    pub cum_qty: String,
    #[serde(rename="AvgPx")]
    pub avg_px: String,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

