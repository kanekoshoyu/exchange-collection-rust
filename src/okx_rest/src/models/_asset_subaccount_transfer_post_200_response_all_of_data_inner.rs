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
pub struct AssetSubaccountTransferPost200ResponseAllOfDataInner {
    #[serde(rename = "transId", skip_serializing_if = "Option::is_none")]
    pub trans_id: Option<String>,
}

impl AssetSubaccountTransferPost200ResponseAllOfDataInner {
    pub fn new() -> AssetSubaccountTransferPost200ResponseAllOfDataInner {
        AssetSubaccountTransferPost200ResponseAllOfDataInner {
            trans_id: None,
        }
    }
}

