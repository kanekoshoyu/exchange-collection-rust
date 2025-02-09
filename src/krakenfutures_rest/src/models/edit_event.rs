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
pub struct EditEvent {
    /// The type of the API order event. Always \"place\".
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Type>,
    #[serde(rename = "old", skip_serializing_if = "Option::is_none")]
    pub old: Option<Box<models::Order>>,
    #[serde(rename = "new", skip_serializing_if = "Option::is_none")]
    pub new: Option<Box<models::Order>>,
    /// The amount of quantity that was removed from the edited order or null if the order is not a reduce only.
    #[serde(rename = "reducedQuantity", skip_serializing_if = "Option::is_none")]
    pub reduced_quantity: Option<f64>,
}

impl EditEvent {
    pub fn new() -> EditEvent {
        EditEvent {
            r#type: None,
            old: None,
            new: None,
            reduced_quantity: None,
        }
    }
}
/// The type of the API order event. Always \"place\".
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "place")]
    Place,
    #[serde(rename = "cancel")]
    Cancel,
    #[serde(rename = "edit")]
    Edit,
    #[serde(rename = "reject")]
    Reject,
    #[serde(rename = "execution")]
    Execution,
}

impl Default for Type {
    fn default() -> Type {
        Self::Place
    }
}

