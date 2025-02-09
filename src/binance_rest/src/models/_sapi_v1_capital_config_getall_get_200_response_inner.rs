/*
 * Binance Public Spot API
 *
 * OpenAPI Specifications for the Binance Public Spot API  API documents:   - [https://github.com/binance/binance-spot-api-docs](https://github.com/binance/binance-spot-api-docs)   - [https://binance-docs.github.io/apidocs/spot/en](https://binance-docs.github.io/apidocs/spot/en)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SapiV1CapitalConfigGetallGet200ResponseInner {
    #[serde(rename = "coin")]
    pub coin: String,
    #[serde(rename = "depositAllEnable")]
    pub deposit_all_enable: bool,
    #[serde(rename = "free")]
    pub free: String,
    #[serde(rename = "freeze")]
    pub freeze: String,
    #[serde(rename = "ipoable")]
    pub ipoable: String,
    #[serde(rename = "ipoing")]
    pub ipoing: String,
    #[serde(rename = "isLegalMoney")]
    pub is_legal_money: bool,
    #[serde(rename = "locked")]
    pub locked: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "networkList")]
    pub network_list: Vec<models::SapiV1CapitalConfigGetallGet200ResponseInnerNetworkListInner>,
    #[serde(rename = "storage")]
    pub storage: String,
    #[serde(rename = "trading")]
    pub trading: bool,
    #[serde(rename = "withdrawAllEnable")]
    pub withdraw_all_enable: bool,
    #[serde(rename = "withdrawing")]
    pub withdrawing: String,
}

impl SapiV1CapitalConfigGetallGet200ResponseInner {
    pub fn new(coin: String, deposit_all_enable: bool, free: String, freeze: String, ipoable: String, ipoing: String, is_legal_money: bool, locked: String, name: String, network_list: Vec<models::SapiV1CapitalConfigGetallGet200ResponseInnerNetworkListInner>, storage: String, trading: bool, withdraw_all_enable: bool, withdrawing: String) -> SapiV1CapitalConfigGetallGet200ResponseInner {
        SapiV1CapitalConfigGetallGet200ResponseInner {
            coin,
            deposit_all_enable,
            free,
            freeze,
            ipoable,
            ipoing,
            is_legal_money,
            locked,
            name,
            network_list,
            storage,
            trading,
            withdraw_all_enable,
            withdrawing,
        }
    }
}

