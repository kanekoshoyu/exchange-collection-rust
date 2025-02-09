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


/// struct for typed errors of method [`public_coin_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PublicCoinGetError {
    Status400(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`public_symbol_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PublicSymbolGetError {
    Status400(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`spot_market_candles_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SpotMarketCandlesGetError {
    Status400(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`spot_market_fills_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SpotMarketFillsGetError {
    Status400(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`spot_market_fills_history_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SpotMarketFillsHistoryGetError {
    Status400(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`spot_market_history_candles_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SpotMarketHistoryCandlesGetError {
    Status400(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`spot_market_merge_depth_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SpotMarketMergeDepthGetError {
    Status400(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`spot_market_orderbook_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SpotMarketOrderbookGetError {
    Status400(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`spot_market_tickers_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SpotMarketTickersGetError {
    Status400(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`spot_market_vip_fee_rate_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SpotMarketVipFeeRateGetError {
    Status400(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}


/// Frequency limit: 3 times/1s (IP).Get spot coin information,supporting both individual and full queries.
pub async fn public_coin_get(configuration: &configuration::Configuration, coin: Option<&str>) -> Result<models::PublicCoinGet200Response, Error<PublicCoinGetError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_coin = coin;

    let uri_str = format!("{}/public/coin", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_coin {
        req_builder = req_builder.query(&[("coin", &param_value.to_string())]);
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
        let entity: Option<PublicCoinGetError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Frequency limit: 20 times/1s (IP).Get spot trading pair information,supporting both individual and full queries
pub async fn public_symbol_get(configuration: &configuration::Configuration, symbol: Option<&str>) -> Result<models::PublicSymbolGet200Response, Error<PublicSymbolGetError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_symbol = symbol;

    let uri_str = format!("{}/public/symbol", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_symbol {
        req_builder = req_builder.query(&[("symbol", &param_value.to_string())]);
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
        let entity: Option<PublicSymbolGetError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Frequency limit: 20 times/1s (IP)
pub async fn spot_market_candles_get(configuration: &configuration::Configuration, symbol: &str, granularity: &str, start_time: Option<&str>, end_time: Option<&str>, limit: Option<i32>) -> Result<models::SpotMarketCandlesGet200Response, Error<SpotMarketCandlesGetError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_symbol = symbol;
    let p_granularity = granularity;
    let p_start_time = start_time;
    let p_end_time = end_time;
    let p_limit = limit;

    let uri_str = format!("{}/spot/market/candles", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    req_builder = req_builder.query(&[("symbol", &p_symbol.to_string())]);
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
        let entity: Option<SpotMarketCandlesGetError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Frequency limit: 10 times/1s (IP)
pub async fn spot_market_fills_get(configuration: &configuration::Configuration, symbol: &str, limit: Option<i32>) -> Result<models::SpotMarketFillsGet200Response, Error<SpotMarketFillsGetError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_symbol = symbol;
    let p_limit = limit;

    let uri_str = format!("{}/spot/market/fills", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

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
        let entity: Option<SpotMarketFillsGetError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Rate limit: 10 req/sec/IP
pub async fn spot_market_fills_history_get(configuration: &configuration::Configuration, symbol: &str, limit: Option<&str>, id_less_than: Option<&str>, start_time: Option<&str>, end_time: Option<&str>) -> Result<models::SpotMarketFillsGet200Response, Error<SpotMarketFillsHistoryGetError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_symbol = symbol;
    let p_limit = limit;
    let p_id_less_than = id_less_than;
    let p_start_time = start_time;
    let p_end_time = end_time;

    let uri_str = format!("{}/spot/market/fills-history", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    req_builder = req_builder.query(&[("symbol", &p_symbol.to_string())]);
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
        let entity: Option<SpotMarketFillsHistoryGetError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Frequency limit: 20 times/1s (IP)
pub async fn spot_market_history_candles_get(configuration: &configuration::Configuration, symbol: &str, granularity: &str, start_time: Option<&str>, end_time: Option<&str>, limit: Option<i32>) -> Result<models::SpotMarketCandlesGet200Response, Error<SpotMarketHistoryCandlesGetError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_symbol = symbol;
    let p_granularity = granularity;
    let p_start_time = start_time;
    let p_end_time = end_time;
    let p_limit = limit;

    let uri_str = format!("{}/spot/market/history-candles", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    req_builder = req_builder.query(&[("symbol", &p_symbol.to_string())]);
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
        let entity: Option<SpotMarketHistoryCandlesGetError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Frequency limit: 20 times/1s (IP)
pub async fn spot_market_merge_depth_get(configuration: &configuration::Configuration, symbol: &str, precision: Option<&str>, limit: Option<&str>) -> Result<models::SpotMarketMergeDepthGet200Response, Error<SpotMarketMergeDepthGetError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_symbol = symbol;
    let p_precision = precision;
    let p_limit = limit;

    let uri_str = format!("{}/spot/market/merge-depth", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    req_builder = req_builder.query(&[("symbol", &p_symbol.to_string())]);
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
        let entity: Option<SpotMarketMergeDepthGetError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Frequency limit: 20 times/1s (IP)
pub async fn spot_market_orderbook_get(configuration: &configuration::Configuration, symbol: &str, r#type: Option<&str>, limit: Option<&str>) -> Result<models::SpotMarketOrderbookGet200Response, Error<SpotMarketOrderbookGetError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_symbol = symbol;
    let p_type = r#type;
    let p_limit = limit;

    let uri_str = format!("{}/spot/market/orderbook", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    req_builder = req_builder.query(&[("symbol", &p_symbol.to_string())]);
    if let Some(ref param_value) = p_type {
        req_builder = req_builder.query(&[("type", &param_value.to_string())]);
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
        let entity: Option<SpotMarketOrderbookGetError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Frequency limit: 20 times/1s (IP).Get Ticker Information,Supports both single and batch queries
pub async fn spot_market_tickers_get(configuration: &configuration::Configuration, symbol: Option<&str>) -> Result<models::SpotMarketTickersGet200Response, Error<SpotMarketTickersGetError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_symbol = symbol;

    let uri_str = format!("{}/spot/market/tickers", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_symbol {
        req_builder = req_builder.query(&[("symbol", &param_value.to_string())]);
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
        let entity: Option<SpotMarketTickersGetError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Frequency limit: 10 times/1s (IP)
pub async fn spot_market_vip_fee_rate_get(configuration: &configuration::Configuration, ) -> Result<models::SpotMarketVipFeeRateGet200Response, Error<SpotMarketVipFeeRateGetError>> {

    let uri_str = format!("{}/spot/market/vip-fee-rate", configuration.base_path);
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
        let entity: Option<SpotMarketVipFeeRateGetError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

