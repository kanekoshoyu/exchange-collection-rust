/*
 * Bitget API
 *
 * Welcome to Bitget Developer document! 
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */


use reqwest;
use serde::{Deserialize, Serialize};
use crate::{apis::ResponseContent, models};
use super::{Error, configuration};


/// struct for typed errors of method [`mix_market_candles_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MixMarketCandlesGetError {
    Status400(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`mix_market_contracts_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MixMarketContractsGetError {
    Status400(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`mix_market_discount_rate_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MixMarketDiscountRateGetError {
    Status400(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`mix_market_exchange_rate_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MixMarketExchangeRateGetError {
    Status400(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`mix_market_fills_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MixMarketFillsGetError {
    Status400(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`mix_market_fills_history_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MixMarketFillsHistoryGetError {
    Status400(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`mix_market_funding_time_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MixMarketFundingTimeGetError {
    Status400(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`mix_market_history_candles_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MixMarketHistoryCandlesGetError {
    Status400(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`mix_market_history_fund_rate_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MixMarketHistoryFundRateGetError {
    Status400(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`mix_market_history_index_candles_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MixMarketHistoryIndexCandlesGetError {
    Status400(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`mix_market_history_mark_candles_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MixMarketHistoryMarkCandlesGetError {
    Status400(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`mix_market_merge_depth_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MixMarketMergeDepthGetError {
    Status400(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`mix_market_open_interest_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MixMarketOpenInterestGetError {
    Status400(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`mix_market_symbol_price_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MixMarketSymbolPriceGetError {
    Status400(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`mix_market_ticker_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MixMarketTickerGetError {
    Status400(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`mix_market_tickers_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MixMarketTickersGetError {
    Status400(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`mix_market_union_interest_rate_history_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MixMarketUnionInterestRateHistoryGetError {
    Status400(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`mix_market_vip_fee_rate_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MixMarketVipFeeRateGetError {
    Status400(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}


/// By default, 100 records are returned. If there is no data, an empty array is returned. The queryable data history varies depending on the k-line granularity.<br>The rules are as follows:<br>1m, 3m, and 5m can be checked for up to one month;<br>15m can be checked for up to 52 days;<br>30m can be searched for up to 62 days;<br>1H can be checked for up to 83 days;<br>2H can be checked for up to 120 days;<br>4H can be checked for up to 240 days;<br>6H can be checked for up to 360 days
pub async fn mix_market_candles_get(configuration: &configuration::Configuration, symbol: &str, product_type: &str, granularity: &str, start_time: Option<&str>, end_time: Option<&str>, k_line_type: Option<&str>, limit: Option<&str>) -> Result<models::SpotMarketCandlesGet200Response, Error<MixMarketCandlesGetError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_symbol = symbol;
    let p_product_type = product_type;
    let p_granularity = granularity;
    let p_start_time = start_time;
    let p_end_time = end_time;
    let p_k_line_type = k_line_type;
    let p_limit = limit;

    let uri_str = format!("{}/mix/market/candles", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    req_builder = req_builder.query(&[("symbol", &p_symbol.to_string())]);
    req_builder = req_builder.query(&[("productType", &p_product_type.to_string())]);
    req_builder = req_builder.query(&[("granularity", &p_granularity.to_string())]);
    if let Some(ref param_value) = p_start_time {
        req_builder = req_builder.query(&[("startTime", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_end_time {
        req_builder = req_builder.query(&[("endTime", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_k_line_type {
        req_builder = req_builder.query(&[("kLineType", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_limit {
        req_builder = req_builder.query(&[("limit", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text().await?;
        let entity: Option<MixMarketCandlesGetError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Frequency limit: 20 times/s, Interface is used to get future contract details.
pub async fn mix_market_contracts_get(configuration: &configuration::Configuration, symbol: &str, product_type: &str) -> Result<models::MixMarketContractsGet200Response, Error<MixMarketContractsGetError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_symbol = symbol;
    let p_product_type = product_type;

    let uri_str = format!("{}/mix/market/contracts", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    req_builder = req_builder.query(&[("symbol", &p_symbol.to_string())]);
    req_builder = req_builder.query(&[("productType", &p_product_type.to_string())]);
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text().await?;
        let entity: Option<MixMarketContractsGetError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Frequency limit: 5 times/1s (User ID)
pub async fn mix_market_discount_rate_get(configuration: &configuration::Configuration, ) -> Result<models::MixMarketDiscountRateGet200Response, Error<MixMarketDiscountRateGetError>> {

    let uri_str = format!("{}/mix/market/discount-rate", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text().await?;
        let entity: Option<MixMarketDiscountRateGetError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Frequency limit: 5 times/1s (User ID)
pub async fn mix_market_exchange_rate_get(configuration: &configuration::Configuration, ) -> Result<models::MixMarketExchangeRateGet200Response, Error<MixMarketExchangeRateGetError>> {

    let uri_str = format!("{}/mix/market/exchange-rate", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text().await?;
        let entity: Option<MixMarketExchangeRateGetError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Get the record of last 100 transactions
pub async fn mix_market_fills_get(configuration: &configuration::Configuration, product_type: &str, symbol: &str, limit: Option<&str>) -> Result<models::MixMarketFillsGet200Response, Error<MixMarketFillsGetError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_product_type = product_type;
    let p_symbol = symbol;
    let p_limit = limit;

    let uri_str = format!("{}/mix/market/fills", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    req_builder = req_builder.query(&[("productType", &p_product_type.to_string())]);
    req_builder = req_builder.query(&[("symbol", &p_symbol.to_string())]);
    if let Some(ref param_value) = p_limit {
        req_builder = req_builder.query(&[("limit", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text().await?;
        let entity: Option<MixMarketFillsGetError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Frequency limit: 10 times/1s (IP)
pub async fn mix_market_fills_history_get(configuration: &configuration::Configuration, symbol: &str, product_type: &str, limit: Option<&str>, id_less_than: Option<&str>, start_time: Option<&str>, end_time: Option<&str>) -> Result<models::MixMarketFillsHistoryGet200Response, Error<MixMarketFillsHistoryGetError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_symbol = symbol;
    let p_product_type = product_type;
    let p_limit = limit;
    let p_id_less_than = id_less_than;
    let p_start_time = start_time;
    let p_end_time = end_time;

    let uri_str = format!("{}/mix/market/fills-history", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    req_builder = req_builder.query(&[("symbol", &p_symbol.to_string())]);
    req_builder = req_builder.query(&[("productType", &p_product_type.to_string())]);
    if let Some(ref param_value) = p_limit {
        req_builder = req_builder.query(&[("limit", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_id_less_than {
        req_builder = req_builder.query(&[("idLessThan", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_start_time {
        req_builder = req_builder.query(&[("startTime", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_end_time {
        req_builder = req_builder.query(&[("endTime", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text().await?;
        let entity: Option<MixMarketFillsHistoryGetError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Frequency limit: 20 times/1s (IP).Get the next settlement time of the contract and the settlement period of the contract.
pub async fn mix_market_funding_time_get(configuration: &configuration::Configuration, symbol: &str, product_type: &str) -> Result<models::MixMarketFundingTimeGet200Response, Error<MixMarketFundingTimeGetError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_symbol = symbol;
    let p_product_type = product_type;

    let uri_str = format!("{}/mix/market/funding-time", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    req_builder = req_builder.query(&[("symbol", &p_symbol.to_string())]);
    req_builder = req_builder.query(&[("productType", &p_product_type.to_string())]);
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text().await?;
        let entity: Option<MixMarketFundingTimeGetError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Frequency limit: 20 times/1s (IP).Query all historical K-line data and return a maximum of 200 pieces of data.
pub async fn mix_market_history_candles_get(configuration: &configuration::Configuration, symbol: &str, product_type: &str, granularity: &str, start_time: Option<&str>, end_time: Option<&str>, k_line_type: Option<&str>, limit: Option<&str>) -> Result<models::SpotMarketCandlesGet200Response, Error<MixMarketHistoryCandlesGetError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_symbol = symbol;
    let p_product_type = product_type;
    let p_granularity = granularity;
    let p_start_time = start_time;
    let p_end_time = end_time;
    let p_k_line_type = k_line_type;
    let p_limit = limit;

    let uri_str = format!("{}/mix/market/history-candles", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    req_builder = req_builder.query(&[("symbol", &p_symbol.to_string())]);
    req_builder = req_builder.query(&[("productType", &p_product_type.to_string())]);
    req_builder = req_builder.query(&[("granularity", &p_granularity.to_string())]);
    if let Some(ref param_value) = p_start_time {
        req_builder = req_builder.query(&[("startTime", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_end_time {
        req_builder = req_builder.query(&[("endTime", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_k_line_type {
        req_builder = req_builder.query(&[("kLineType", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_limit {
        req_builder = req_builder.query(&[("limit", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text().await?;
        let entity: Option<MixMarketHistoryCandlesGetError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Frequency limit: 20 times/s, Get the historical funding rate of the contract
pub async fn mix_market_history_fund_rate_get(configuration: &configuration::Configuration, symbol: &str, product_type: &str, page_size: Option<&str>, page_no: Option<&str>) -> Result<models::MixMarketHistoryFundRateGet200Response, Error<MixMarketHistoryFundRateGetError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_symbol = symbol;
    let p_product_type = product_type;
    let p_page_size = page_size;
    let p_page_no = page_no;

    let uri_str = format!("{}/mix/market/history-fund-rate", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    req_builder = req_builder.query(&[("symbol", &p_symbol.to_string())]);
    req_builder = req_builder.query(&[("productType", &p_product_type.to_string())]);
    if let Some(ref param_value) = p_page_size {
        req_builder = req_builder.query(&[("pageSize", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_page_no {
        req_builder = req_builder.query(&[("pageNo", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text().await?;
        let entity: Option<MixMarketHistoryFundRateGetError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Frequency limit: 20 times/1s (IP).Query the historical K-line data of contract index price, and return a maximum of 200 pieces of data.
pub async fn mix_market_history_index_candles_get(configuration: &configuration::Configuration, symbol: &str, product_type: &str, granularity: &str, start_time: Option<&str>, end_time: Option<&str>, limit: Option<&str>) -> Result<models::SpotMarketCandlesGet200Response, Error<MixMarketHistoryIndexCandlesGetError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_symbol = symbol;
    let p_product_type = product_type;
    let p_granularity = granularity;
    let p_start_time = start_time;
    let p_end_time = end_time;
    let p_limit = limit;

    let uri_str = format!("{}/mix/market/history-index-candles", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    req_builder = req_builder.query(&[("symbol", &p_symbol.to_string())]);
    req_builder = req_builder.query(&[("productType", &p_product_type.to_string())]);
    req_builder = req_builder.query(&[("granularity", &p_granularity.to_string())]);
    if let Some(ref param_value) = p_start_time {
        req_builder = req_builder.query(&[("startTime", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_end_time {
        req_builder = req_builder.query(&[("endTime", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_limit {
        req_builder = req_builder.query(&[("limit", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text().await?;
        let entity: Option<MixMarketHistoryIndexCandlesGetError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Frequency limit: 20 times/1s (IP).Query the historical K-line data of contract index price, and return a maximum of 200 pieces of data.
pub async fn mix_market_history_mark_candles_get(configuration: &configuration::Configuration, symbol: &str, product_type: &str, granularity: &str, start_time: Option<&str>, end_time: Option<&str>, limit: Option<&str>) -> Result<models::SpotMarketCandlesGet200Response, Error<MixMarketHistoryMarkCandlesGetError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_symbol = symbol;
    let p_product_type = product_type;
    let p_granularity = granularity;
    let p_start_time = start_time;
    let p_end_time = end_time;
    let p_limit = limit;

    let uri_str = format!("{}/mix/market/history-mark-candles", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    req_builder = req_builder.query(&[("symbol", &p_symbol.to_string())]);
    req_builder = req_builder.query(&[("productType", &p_product_type.to_string())]);
    req_builder = req_builder.query(&[("granularity", &p_granularity.to_string())]);
    if let Some(ref param_value) = p_start_time {
        req_builder = req_builder.query(&[("startTime", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_end_time {
        req_builder = req_builder.query(&[("endTime", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_limit {
        req_builder = req_builder.query(&[("limit", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text().await?;
        let entity: Option<MixMarketHistoryMarkCandlesGetError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Frequency limit: 20 times/1s (User ID)
pub async fn mix_market_merge_depth_get(configuration: &configuration::Configuration, symbol: &str, product_type: &str, precision: Option<&str>, limit: Option<&str>) -> Result<models::MixMarketMergeDepthGet200Response, Error<MixMarketMergeDepthGetError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_symbol = symbol;
    let p_product_type = product_type;
    let p_precision = precision;
    let p_limit = limit;

    let uri_str = format!("{}/mix/market/merge-depth", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    req_builder = req_builder.query(&[("symbol", &p_symbol.to_string())]);
    req_builder = req_builder.query(&[("productType", &p_product_type.to_string())]);
    if let Some(ref param_value) = p_precision {
        req_builder = req_builder.query(&[("precision", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_limit {
        req_builder = req_builder.query(&[("limit", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text().await?;
        let entity: Option<MixMarketMergeDepthGetError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Frequency limit: 20 times/1s (IP).Get the total positions of a certain trading pair on the platform.
pub async fn mix_market_open_interest_get(configuration: &configuration::Configuration, symbol: &str, product_type: &str) -> Result<models::MixMarketOpenInterestGet200Response, Error<MixMarketOpenInterestGetError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_symbol = symbol;
    let p_product_type = product_type;

    let uri_str = format!("{}/mix/market/open-interest", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    req_builder = req_builder.query(&[("symbol", &p_symbol.to_string())]);
    req_builder = req_builder.query(&[("productType", &p_product_type.to_string())]);
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text().await?;
        let entity: Option<MixMarketOpenInterestGetError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Frequency limit: 20 times/s, frequency is limited according to user ID.Get market/index/mark prices.
pub async fn mix_market_symbol_price_get(configuration: &configuration::Configuration, symbol: &str, product_type: &str) -> Result<models::MixMarketSymbolPriceGet200Response, Error<MixMarketSymbolPriceGetError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_symbol = symbol;
    let p_product_type = product_type;

    let uri_str = format!("{}/mix/market/symbol-price", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    req_builder = req_builder.query(&[("symbol", &p_symbol.to_string())]);
    req_builder = req_builder.query(&[("productType", &p_product_type.to_string())]);
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text().await?;
        let entity: Option<MixMarketSymbolPriceGetError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Get ticker data of the given 'productType' and 'symbol'
pub async fn mix_market_ticker_get(configuration: &configuration::Configuration, symbol: &str, product_type: &str) -> Result<models::MixMarketTickerGet200Response, Error<MixMarketTickerGetError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_symbol = symbol;
    let p_product_type = product_type;

    let uri_str = format!("{}/mix/market/ticker", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    req_builder = req_builder.query(&[("symbol", &p_symbol.to_string())]);
    req_builder = req_builder.query(&[("productType", &p_product_type.to_string())]);
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text().await?;
        let entity: Option<MixMarketTickerGetError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Get ticker data of the given 'productType'
pub async fn mix_market_tickers_get(configuration: &configuration::Configuration, product_type: &str) -> Result<models::MixMarketTickerGet200Response, Error<MixMarketTickersGetError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_product_type = product_type;

    let uri_str = format!("{}/mix/market/tickers", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    req_builder = req_builder.query(&[("productType", &p_product_type.to_string())]);
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text().await?;
        let entity: Option<MixMarketTickersGetError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Frequency limit: 5 times/1s (User ID)
pub async fn mix_market_union_interest_rate_history_get(configuration: &configuration::Configuration, coin: &str) -> Result<models::MixMarketUnionInterestRateHistoryGet200Response, Error<MixMarketUnionInterestRateHistoryGetError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_coin = coin;

    let uri_str = format!("{}/mix/market/union-interest-rate-history", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    req_builder = req_builder.query(&[("coin", &p_coin.to_string())]);
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text().await?;
        let entity: Option<MixMarketUnionInterestRateHistoryGetError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Frequency limit: 10 times/1s (User ID)
pub async fn mix_market_vip_fee_rate_get(configuration: &configuration::Configuration, ) -> Result<models::MixMarketVipFeeRateGet200Response, Error<MixMarketVipFeeRateGetError>> {

    let uri_str = format!("{}/mix/market/vip-fee-rate", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text().await?;
        let entity: Option<MixMarketVipFeeRateGetError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

