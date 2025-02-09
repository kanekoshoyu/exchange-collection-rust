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
pub struct MarginAccount {
    /// The type of the account (always marginAccount).
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Type>,
    /// The currency of the account.
    #[serde(rename = "currency", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub currency: Option<Option<String>>,
    /// A structure containing account balances.
    #[serde(rename = "balances", skip_serializing_if = "Option::is_none")]
    pub balances: Option<std::collections::HashMap<String, f64>>,
    #[serde(rename = "auxiliary", skip_serializing_if = "Option::is_none")]
    pub auxiliary: Option<Box<models::MarginAccountAuxiliary>>,
    #[serde(rename = "marginRequirements", skip_serializing_if = "Option::is_none")]
    pub margin_requirements: Option<Box<models::MarginAccountMarginRequirements>>,
    #[serde(rename = "triggerEstimates", skip_serializing_if = "Option::is_none")]
    pub trigger_estimates: Option<Box<models::MarginAccountTriggerEstimates>>,
}

impl MarginAccount {
    pub fn new() -> MarginAccount {
        MarginAccount {
            r#type: None,
            currency: None,
            balances: None,
            auxiliary: None,
            margin_requirements: None,
            trigger_estimates: None,
        }
    }
}
/// The type of the account (always marginAccount).
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "marginAccount")]
    MarginAccount,
}

impl Default for Type {
    fn default() -> Type {
        Self::MarginAccount
    }
}

