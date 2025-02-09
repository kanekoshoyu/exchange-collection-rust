#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// PerpetualFuturesPosition represents a PerpetualFuturesPosition model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct PerpetualFuturesPosition {
    #[serde(rename="product_id", skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    #[serde(rename="portfolio_uuid", skip_serializing_if = "Option::is_none")]
    pub portfolio_uuid: Option<String>,
    #[serde(rename="vwap", skip_serializing_if = "Option::is_none")]
    pub vwap: Option<String>,
    #[serde(rename="entry_vwap", skip_serializing_if = "Option::is_none")]
    pub entry_vwap: Option<String>,
    #[serde(rename="position_side", skip_serializing_if = "Option::is_none")]
    pub position_side: Option<String>,
    #[serde(rename="margin_type", skip_serializing_if = "Option::is_none")]
    pub margin_type: Option<String>,
    #[serde(rename="net_size", skip_serializing_if = "Option::is_none")]
    pub net_size: Option<String>,
    #[serde(rename="buy_order_size", skip_serializing_if = "Option::is_none")]
    pub buy_order_size: Option<String>,
    #[serde(rename="sell_order_size", skip_serializing_if = "Option::is_none")]
    pub sell_order_size: Option<String>,
    #[serde(rename="leverage", skip_serializing_if = "Option::is_none")]
    pub leverage: Option<String>,
    #[serde(rename="mark_price", skip_serializing_if = "Option::is_none")]
    pub mark_price: Option<String>,
    #[serde(rename="liquidation_price", skip_serializing_if = "Option::is_none")]
    pub liquidation_price: Option<String>,
    #[serde(rename="im_notional", skip_serializing_if = "Option::is_none")]
    pub im_notional: Option<String>,
    #[serde(rename="mm_notional", skip_serializing_if = "Option::is_none")]
    pub mm_notional: Option<String>,
    #[serde(rename="position_notional", skip_serializing_if = "Option::is_none")]
    pub position_notional: Option<String>,
    #[serde(rename="unrealized_pnl", skip_serializing_if = "Option::is_none")]
    pub unrealized_pnl: Option<String>,
    #[serde(rename="aggregated_pnl", skip_serializing_if = "Option::is_none")]
    pub aggregated_pnl: Option<String>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

