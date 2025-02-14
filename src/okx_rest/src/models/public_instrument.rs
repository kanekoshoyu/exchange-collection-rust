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
pub struct PublicInstrument {
    /// Instrument type. Valid values: - SPOT: Spot - MARGIN: Margin - SWAP: Perpetual Futures - FUTURES: Expiry Futures - OPTION: Option 
    #[serde(rename = "instType", skip_serializing_if = "Option::is_none")]
    pub inst_type: Option<String>,
    /// Instrument ID, e.g., BTC-USD-SWAP.
    #[serde(rename = "instId", skip_serializing_if = "Option::is_none")]
    pub inst_id: Option<String>,
    /// Underlying, e.g., BTC-USD. Only applicable to MARGIN/FUTURES/SWAP/OPTION. 
    #[serde(rename = "uly", skip_serializing_if = "Option::is_none")]
    pub uly: Option<String>,
    /// Instrument family, e.g., BTC-USD. Only applicable to MARGIN/FUTURES/SWAP/OPTION. 
    #[serde(rename = "instFamily", skip_serializing_if = "Option::is_none")]
    pub inst_family: Option<String>,
    /// Currency category. Note: this parameter is already deprecated. 
    #[serde(rename = "category", skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    /// Base currency, e.g., BTC in BTC-USDT. Only applicable to SPOT/MARGIN. 
    #[serde(rename = "baseCcy", skip_serializing_if = "Option::is_none")]
    pub base_ccy: Option<String>,
    /// Quote currency, e.g., USDT in BTC-USDT. Only applicable to SPOT/MARGIN. 
    #[serde(rename = "quoteCcy", skip_serializing_if = "Option::is_none")]
    pub quote_ccy: Option<String>,
    /// Settlement and margin currency, e.g., BTC. Only applicable to FUTURES/SWAP/OPTION. 
    #[serde(rename = "settleCcy", skip_serializing_if = "Option::is_none")]
    pub settle_ccy: Option<String>,
    /// Contract value. Only applicable to FUTURES/SWAP/OPTION. 
    #[serde(rename = "ctVal", skip_serializing_if = "Option::is_none")]
    pub ct_val: Option<String>,
    /// Contract multiplier. Only applicable to FUTURES/SWAP/OPTION. 
    #[serde(rename = "ctMult", skip_serializing_if = "Option::is_none")]
    pub ct_mult: Option<String>,
    /// Contract value currency. Only applicable to FUTURES/SWAP/OPTION. 
    #[serde(rename = "ctValCcy", skip_serializing_if = "Option::is_none")]
    pub ct_val_ccy: Option<String>,
    /// Option type: - C: Call - P: Put Only applicable to OPTION. 
    #[serde(rename = "optType", skip_serializing_if = "Option::is_none")]
    pub opt_type: Option<String>,
    /// Strike price. Only applicable to OPTION. 
    #[serde(rename = "stk", skip_serializing_if = "Option::is_none")]
    pub stk: Option<String>,
    /// Listing time in Unix timestamp format in milliseconds, e.g., 1597026383085.
    #[serde(rename = "listTime", skip_serializing_if = "Option::is_none")]
    pub list_time: Option<String>,
    /// The end time of call auction in Unix timestamp format in milliseconds, e.g., 1597026383085. Only applicable to SPOT listed through call auctions, returns \"\" in other cases. 
    #[serde(rename = "auctionEndTime", skip_serializing_if = "Option::is_none")]
    pub auction_end_time: Option<String>,
    /// Expiry time. Applicable to SPOT/MARGIN/FUTURES/SWAP/OPTION. For FUTURES/OPTION, it is the natural delivery/exercise time. For SPOT/MARGIN/FUTURES/SWAP manual offline, it is the instrument offline time. 
    #[serde(rename = "expTime", skip_serializing_if = "Option::is_none")]
    pub exp_time: Option<String>,
    /// Maximum leverage. Not applicable to SPOT, OPTION.
    #[serde(rename = "lever", skip_serializing_if = "Option::is_none")]
    pub lever: Option<String>,
    /// Tick size, e.g., 0.0001. For Option, it is the minimum tick size among tick bands. Use \"Get option tick bands\" to retrieve tick bands. 
    #[serde(rename = "tickSz", skip_serializing_if = "Option::is_none")]
    pub tick_sz: Option<String>,
    /// Lot size. - For derivatives contracts, the value is the number of contracts. - For SPOT/MARGIN, the value is the quantity in base currency. 
    #[serde(rename = "lotSz", skip_serializing_if = "Option::is_none")]
    pub lot_sz: Option<String>,
    /// Minimum order size. - For derivatives contracts, the value is the number of contracts. - For SPOT/MARGIN, the value is the quantity in base currency. 
    #[serde(rename = "minSz", skip_serializing_if = "Option::is_none")]
    pub min_sz: Option<String>,
    /// Contract type. Valid values: - linear: Linear contract - inverse: Inverse contract Only applicable to FUTURES/SWAP. 
    #[serde(rename = "ctType", skip_serializing_if = "Option::is_none")]
    pub ct_type: Option<String>,
    /// Alias. Valid values: - this_week - next_week - this_month - next_month - quarter - next_quarter Only applicable to FUTURES. Not recommended for use; use `expTime` to determine delivery time. 
    #[serde(rename = "alias", skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
    /// Instrument status. Valid values: - live - suspend - preopen: Preopen state before Futures and Options contracts are live. - test: Test pairs, cannot be traded. 
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// Trading rule types. Valid values: - normal: Normal trading - pre_market: Pre-market trading. 
    #[serde(rename = "ruleType", skip_serializing_if = "Option::is_none")]
    pub rule_type: Option<String>,
    /// The maximum order quantity of a single limit order. - For derivatives contracts, the value is the number of contracts. - For SPOT/MARGIN, the value is the quantity in base currency. 
    #[serde(rename = "maxLmtSz", skip_serializing_if = "Option::is_none")]
    pub max_lmt_sz: Option<String>,
    /// The maximum order quantity of a single market order. - For derivatives contracts, the value is the number of contracts. - For SPOT/MARGIN, the value is the quantity in USDT. 
    #[serde(rename = "maxMktSz", skip_serializing_if = "Option::is_none")]
    pub max_mkt_sz: Option<String>,
    /// Maximum USD amount for a single limit order.
    #[serde(rename = "maxLmtAmt", skip_serializing_if = "Option::is_none")]
    pub max_lmt_amt: Option<String>,
    /// Maximum USD amount for a single market order. Only applicable to SPOT/MARGIN. 
    #[serde(rename = "maxMktAmt", skip_serializing_if = "Option::is_none")]
    pub max_mkt_amt: Option<String>,
    /// The maximum order quantity of a single TWAP order. - For derivatives contracts, the value is the number of contracts. - For SPOT/MARGIN, the value is the quantity in base currency. The minimum order quantity of a single TWAP order is `minSz * 2`. 
    #[serde(rename = "maxTwapSz", skip_serializing_if = "Option::is_none")]
    pub max_twap_sz: Option<String>,
    /// The maximum order quantity of a single iceberg order. - For derivatives contracts, the value is the number of contracts. - For SPOT/MARGIN, the value is the quantity in base currency. 
    #[serde(rename = "maxIcebergSz", skip_serializing_if = "Option::is_none")]
    pub max_iceberg_sz: Option<String>,
    /// The maximum order quantity of a single trigger order. - For derivatives contracts, the value is the number of contracts. - For SPOT/MARGIN, the value is the quantity in base currency. 
    #[serde(rename = "maxTriggerSz", skip_serializing_if = "Option::is_none")]
    pub max_trigger_sz: Option<String>,
    /// The maximum order quantity of a single stop market order. - For derivatives contracts, the value is the number of contracts. - For SPOT/MARGIN, the value is the quantity in USDT. 
    #[serde(rename = "maxStopSz", skip_serializing_if = "Option::is_none")]
    pub max_stop_sz: Option<String>,
}

impl PublicInstrument {
    pub fn new() -> PublicInstrument {
        PublicInstrument {
            inst_type: None,
            inst_id: None,
            uly: None,
            inst_family: None,
            category: None,
            base_ccy: None,
            quote_ccy: None,
            settle_ccy: None,
            ct_val: None,
            ct_mult: None,
            ct_val_ccy: None,
            opt_type: None,
            stk: None,
            list_time: None,
            auction_end_time: None,
            exp_time: None,
            lever: None,
            tick_sz: None,
            lot_sz: None,
            min_sz: None,
            ct_type: None,
            alias: None,
            state: None,
            rule_type: None,
            max_lmt_sz: None,
            max_mkt_sz: None,
            max_lmt_amt: None,
            max_mkt_amt: None,
            max_twap_sz: None,
            max_iceberg_sz: None,
            max_trigger_sz: None,
            max_stop_sz: None,
        }
    }
}

