/*
 * Kraken API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RejectEvent {
    /// Unique id of order
    #[serde(rename = "uid", skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
    #[serde(rename = "order", skip_serializing_if = "Option::is_none")]
    pub order: Option<Box<models::Order>>,
    /// The rejection reason for the order.
    #[serde(rename = "reason", skip_serializing_if = "Option::is_none")]
    pub reason: Option<Reason>,
}

impl RejectEvent {
    pub fn new() -> RejectEvent {
        RejectEvent {
            uid: None,
            order: None,
            reason: None,
        }
    }
}
/// The rejection reason for the order.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Reason {
    #[serde(rename = "postWouldExecute")]
    PostWouldExecute,
    #[serde(rename = "iocWouldNotExecute")]
    IocWouldNotExecute,
    #[serde(rename = "wouldNotReducePosition")]
    WouldNotReducePosition,
    #[serde(rename = "orderForEditNotFound")]
    OrderForEditNotFound,
}

impl Default for Reason {
    fn default() -> Reason {
        Self::PostWouldExecute
    }
}

