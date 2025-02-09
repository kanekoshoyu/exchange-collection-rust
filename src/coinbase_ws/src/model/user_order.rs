#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// UserOrder represents a UserOrder model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct UserOrder {
    #[serde(rename="avg_price", skip_serializing_if = "Option::is_none")]
    pub avg_price: Option<String>,
    #[serde(rename="cancel_reason", skip_serializing_if = "Option::is_none")]
    pub cancel_reason: Option<String>,
    #[serde(rename="client_order_id", skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<String>,
    #[serde(rename="completion_percentage", skip_serializing_if = "Option::is_none")]
    pub completion_percentage: Option<String>,
    #[serde(rename="contract_expiry_type", skip_serializing_if = "Option::is_none")]
    pub contract_expiry_type: Option<Box<ExpiryEnum>>,
    #[serde(rename="cumulative_quantity", skip_serializing_if = "Option::is_none")]
    pub cumulative_quantity: Option<String>,
    #[serde(rename="filled_value", skip_serializing_if = "Option::is_none")]
    pub filled_value: Option<String>,
    #[serde(rename="leaves_quantity", skip_serializing_if = "Option::is_none")]
    pub leaves_quantity: Option<String>,
    #[serde(rename="limit_price", skip_serializing_if = "Option::is_none")]
    pub limit_price: Option<String>,
    #[serde(rename="number_of_fills", skip_serializing_if = "Option::is_none")]
    pub number_of_fills: Option<i32>,
    #[serde(rename="order_id", skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(rename="order_side", skip_serializing_if = "Option::is_none")]
    pub order_side: Option<String>,
    #[serde(rename="order_type", skip_serializing_if = "Option::is_none")]
    pub order_type: Option<Box<OrderEnum>>,
    #[serde(rename="outstanding_hold_amount", skip_serializing_if = "Option::is_none")]
    pub outstanding_hold_amount: Option<String>,
    #[serde(rename="post_only", skip_serializing_if = "Option::is_none")]
    pub post_only: Option<String>,
    #[serde(rename="product_id", skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    #[serde(rename="product_type", skip_serializing_if = "Option::is_none")]
    pub product_type: Option<Box<ProductEnum>>,
    #[serde(rename="reject_reason", skip_serializing_if = "Option::is_none")]
    pub reject_reason: Option<String>,
    #[serde(rename="retail_portfolio_id", skip_serializing_if = "Option::is_none")]
    pub retail_portfolio_id: Option<String>,
    #[serde(rename="risk_managed_by", skip_serializing_if = "Option::is_none")]
    pub risk_managed_by: Option<Box<RiskEnum>>,
    #[serde(rename="status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Box<StatusEnum>>,
    #[serde(rename="stop_price", skip_serializing_if = "Option::is_none")]
    pub stop_price: Option<String>,
    #[serde(rename="time_in_force", skip_serializing_if = "Option::is_none")]
    pub time_in_force: Option<Box<TimeEnum>>,
    #[serde(rename="total_fees", skip_serializing_if = "Option::is_none")]
    pub total_fees: Option<String>,
    #[serde(rename="total_value_after_fees", skip_serializing_if = "Option::is_none")]
    pub total_value_after_fees: Option<String>,
    #[serde(rename="trigger_status", skip_serializing_if = "Option::is_none")]
    pub trigger_status: Option<Box<TriggerEnum>>,
    #[serde(rename="creation_time", skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<String>,
    #[serde(rename="end_time", skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[serde(rename="start_time", skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

