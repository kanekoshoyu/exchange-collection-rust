#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// CalendarPushData represents a CalendarPushData model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct CalendarPushData {
    #[serde(rename="event", skip_serializing_if = "Option::is_none")]
    pub event: Option<String>,
    #[serde(rename="region", skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(rename="category", skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(rename="actual", skip_serializing_if = "Option::is_none")]
    pub actual: Option<String>,
    #[serde(rename="previous", skip_serializing_if = "Option::is_none")]
    pub previous: Option<String>,
    #[serde(rename="forecast", skip_serializing_if = "Option::is_none")]
    pub forecast: Option<String>,
    #[serde(rename="prevInitial", skip_serializing_if = "Option::is_none")]
    pub prev_initial: Option<String>,
    #[serde(rename="date", skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    #[serde(rename="refDate", skip_serializing_if = "Option::is_none")]
    pub ref_date: Option<String>,
    #[serde(rename="calendarId", skip_serializing_if = "Option::is_none")]
    pub calendar_id: Option<String>,
    #[serde(rename="unit", skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[serde(rename="ccy", skip_serializing_if = "Option::is_none")]
    pub ccy: Option<String>,
    #[serde(rename="importance", skip_serializing_if = "Option::is_none")]
    pub importance: Option<String>,
    #[serde(rename="ts", skip_serializing_if = "Option::is_none")]
    pub ts: Option<String>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

