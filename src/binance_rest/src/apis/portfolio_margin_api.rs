/*
 * Binance Public Spot API
 *
 * OpenAPI Specifications for the Binance Public Spot API  API documents:   - [https://github.com/binance/binance-spot-api-docs](https://github.com/binance/binance-spot-api-docs)   - [https://binance-docs.github.io/apidocs/spot/en](https://binance-docs.github.io/apidocs/spot/en)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */


use reqwest;
use serde::{Deserialize, Serialize};
use crate::{apis::ResponseContent, models};
use super::{Error, configuration};


/// struct for typed errors of method [`sapi_v1_portfolio_account_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SapiV1PortfolioAccountGetError {
    Status400(models::Error),
    Status401(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`sapi_v1_portfolio_asset_collection_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SapiV1PortfolioAssetCollectionPostError {
    Status400(models::Error),
    Status401(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`sapi_v1_portfolio_asset_index_price_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SapiV1PortfolioAssetIndexPriceGetError {
    Status400(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`sapi_v1_portfolio_auto_collection_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SapiV1PortfolioAutoCollectionPostError {
    Status400(models::Error),
    Status401(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`sapi_v1_portfolio_bnb_transfer_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SapiV1PortfolioBnbTransferPostError {
    Status400(models::Error),
    Status401(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`sapi_v1_portfolio_collateral_rate_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SapiV1PortfolioCollateralRateGetError {
    Status400(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`sapi_v1_portfolio_interest_history_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SapiV1PortfolioInterestHistoryGetError {
    Status400(models::Error),
    Status401(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`sapi_v1_portfolio_margin_asset_leverage_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SapiV1PortfolioMarginAssetLeverageGetError {
    Status400(models::Error),
    Status401(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`sapi_v1_portfolio_pm_loan_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SapiV1PortfolioPmLoanGetError {
    Status400(models::Error),
    Status401(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`sapi_v1_portfolio_repay_futures_negative_balance_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SapiV1PortfolioRepayFuturesNegativeBalancePostError {
    Status400(models::Error),
    Status401(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`sapi_v1_portfolio_repay_futures_switch_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SapiV1PortfolioRepayFuturesSwitchGetError {
    Status400(models::Error),
    Status401(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`sapi_v1_portfolio_repay_futures_switch_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SapiV1PortfolioRepayFuturesSwitchPostError {
    Status400(models::Error),
    Status401(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`sapi_v1_portfolio_repay_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SapiV1PortfolioRepayPostError {
    Status400(models::Error),
    Status401(models::Error),
    UnknownValue(serde_json::Value),
}


/// Get the account info  'Weight(IP): 1'
pub async fn sapi_v1_portfolio_account_get(configuration: &configuration::Configuration, timestamp: i64, signature: &str, recv_window: Option<i64>) -> Result<models::SapiV1PortfolioAccountGet200Response, Error<SapiV1PortfolioAccountGetError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_timestamp = timestamp;
    let p_signature = signature;
    let p_recv_window = recv_window;

    let uri_str = format!("{}/sapi/v1/portfolio/account", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_recv_window {
        req_builder = req_builder.query(&[("recvWindow", &param_value.to_string())]);
    }
    req_builder = req_builder.query(&[("timestamp", &p_timestamp.to_string())]);
    req_builder = req_builder.query(&[("signature", &p_signature.to_string())]);
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("X-MBX-APIKEY", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text().await?;
        let entity: Option<SapiV1PortfolioAccountGetError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Transfers specific asset from Futures Account to Margin account  Weight(IP): 60
pub async fn sapi_v1_portfolio_asset_collection_post(configuration: &configuration::Configuration, asset: &str, timestamp: i64, signature: &str, recv_window: Option<i64>) -> Result<models::SapiV1PortfolioAutoCollectionPost200Response, Error<SapiV1PortfolioAssetCollectionPostError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_asset = asset;
    let p_timestamp = timestamp;
    let p_signature = signature;
    let p_recv_window = recv_window;

    let uri_str = format!("{}/sapi/v1/portfolio/asset-collection", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    req_builder = req_builder.query(&[("asset", &p_asset.to_string())]);
    if let Some(ref param_value) = p_recv_window {
        req_builder = req_builder.query(&[("recvWindow", &param_value.to_string())]);
    }
    req_builder = req_builder.query(&[("timestamp", &p_timestamp.to_string())]);
    req_builder = req_builder.query(&[("signature", &p_signature.to_string())]);
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("X-MBX-APIKEY", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text().await?;
        let entity: Option<SapiV1PortfolioAssetCollectionPostError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Query Portfolio Margin Asset Index Price  Weight(IP): - 1 if send asset - 50 if not send asset
pub async fn sapi_v1_portfolio_asset_index_price_get(configuration: &configuration::Configuration, asset: Option<&str>) -> Result<Vec<models::SapiV1PortfolioAssetIndexPriceGet200ResponseInner>, Error<SapiV1PortfolioAssetIndexPriceGetError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_asset = asset;

    let uri_str = format!("{}/sapi/v1/portfolio/asset-index-price", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_asset {
        req_builder = req_builder.query(&[("asset", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("X-MBX-APIKEY", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text().await?;
        let entity: Option<SapiV1PortfolioAssetIndexPriceGetError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Transfers all assets from Futures Account to Margin account  Weight(IP): 1500
pub async fn sapi_v1_portfolio_auto_collection_post(configuration: &configuration::Configuration, timestamp: i64, signature: &str, recv_window: Option<i64>) -> Result<models::SapiV1PortfolioAutoCollectionPost200Response, Error<SapiV1PortfolioAutoCollectionPostError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_timestamp = timestamp;
    let p_signature = signature;
    let p_recv_window = recv_window;

    let uri_str = format!("{}/sapi/v1/portfolio/auto-collection", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref param_value) = p_recv_window {
        req_builder = req_builder.query(&[("recvWindow", &param_value.to_string())]);
    }
    req_builder = req_builder.query(&[("timestamp", &p_timestamp.to_string())]);
    req_builder = req_builder.query(&[("signature", &p_signature.to_string())]);
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("X-MBX-APIKEY", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text().await?;
        let entity: Option<SapiV1PortfolioAutoCollectionPostError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// BNB transfer can be between Margin Account and USDM Account  Weight(IP): 1500
pub async fn sapi_v1_portfolio_bnb_transfer_post(configuration: &configuration::Configuration, transfer_side: &str, amount: f64, timestamp: i64, signature: &str, recv_window: Option<i64>) -> Result<models::SapiV1MarginBorrowRepayPost200Response, Error<SapiV1PortfolioBnbTransferPostError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_transfer_side = transfer_side;
    let p_amount = amount;
    let p_timestamp = timestamp;
    let p_signature = signature;
    let p_recv_window = recv_window;

    let uri_str = format!("{}/sapi/v1/portfolio/bnb-transfer", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    req_builder = req_builder.query(&[("transferSide", &p_transfer_side.to_string())]);
    req_builder = req_builder.query(&[("amount", &p_amount.to_string())]);
    if let Some(ref param_value) = p_recv_window {
        req_builder = req_builder.query(&[("recvWindow", &param_value.to_string())]);
    }
    req_builder = req_builder.query(&[("timestamp", &p_timestamp.to_string())]);
    req_builder = req_builder.query(&[("signature", &p_signature.to_string())]);
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("X-MBX-APIKEY", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text().await?;
        let entity: Option<SapiV1PortfolioBnbTransferPostError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Portfolio Margin Collateral Rate.  Weight(IP): 50
pub async fn sapi_v1_portfolio_collateral_rate_get(configuration: &configuration::Configuration, ) -> Result<Vec<models::SapiV1PortfolioCollateralRateGet200ResponseInner>, Error<SapiV1PortfolioCollateralRateGetError>> {

    let uri_str = format!("{}/sapi/v1/portfolio/collateralRate", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("X-MBX-APIKEY", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text().await?;
        let entity: Option<SapiV1PortfolioCollateralRateGetError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Query interest history of negative balance for portfolio margin.  Weight(IP): 50
pub async fn sapi_v1_portfolio_interest_history_get(configuration: &configuration::Configuration, asset: &str, timestamp: i64, signature: &str, start_time: Option<i64>, end_time: Option<i64>, size: Option<i32>, recv_window: Option<i64>) -> Result<Vec<models::SapiV1PortfolioInterestHistoryGet200ResponseInner>, Error<SapiV1PortfolioInterestHistoryGetError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_asset = asset;
    let p_timestamp = timestamp;
    let p_signature = signature;
    let p_start_time = start_time;
    let p_end_time = end_time;
    let p_size = size;
    let p_recv_window = recv_window;

    let uri_str = format!("{}/sapi/v1/portfolio/interest-history", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    req_builder = req_builder.query(&[("asset", &p_asset.to_string())]);
    if let Some(ref param_value) = p_start_time {
        req_builder = req_builder.query(&[("startTime", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_end_time {
        req_builder = req_builder.query(&[("endTime", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_size {
        req_builder = req_builder.query(&[("size", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_recv_window {
        req_builder = req_builder.query(&[("recvWindow", &param_value.to_string())]);
    }
    req_builder = req_builder.query(&[("timestamp", &p_timestamp.to_string())]);
    req_builder = req_builder.query(&[("signature", &p_signature.to_string())]);
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("X-MBX-APIKEY", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text().await?;
        let entity: Option<SapiV1PortfolioInterestHistoryGetError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Weight(IP): 50
pub async fn sapi_v1_portfolio_margin_asset_leverage_get(configuration: &configuration::Configuration, ) -> Result<Vec<models::SapiV1PortfolioMarginAssetLeverageGet200ResponseInner>, Error<SapiV1PortfolioMarginAssetLeverageGetError>> {

    let uri_str = format!("{}/sapi/v1/portfolio/margin-asset-leverage", configuration.base_path);
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
        let entity: Option<SapiV1PortfolioMarginAssetLeverageGetError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Query Portfolio Margin Bankruptcy Loan Amount.  Weight(UID): 500
pub async fn sapi_v1_portfolio_pm_loan_get(configuration: &configuration::Configuration, timestamp: i64, signature: &str, recv_window: Option<i64>) -> Result<models::SapiV1PortfolioPmLoanGet200Response, Error<SapiV1PortfolioPmLoanGetError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_timestamp = timestamp;
    let p_signature = signature;
    let p_recv_window = recv_window;

    let uri_str = format!("{}/sapi/v1/portfolio/pmLoan", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_recv_window {
        req_builder = req_builder.query(&[("recvWindow", &param_value.to_string())]);
    }
    req_builder = req_builder.query(&[("timestamp", &p_timestamp.to_string())]);
    req_builder = req_builder.query(&[("signature", &p_signature.to_string())]);
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("X-MBX-APIKEY", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text().await?;
        let entity: Option<SapiV1PortfolioPmLoanGetError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Repay futures Negative Balance  Weight(IP): 1500
pub async fn sapi_v1_portfolio_repay_futures_negative_balance_post(configuration: &configuration::Configuration, timestamp: i64, signature: &str, recv_window: Option<i64>) -> Result<models::SapiV1PortfolioAutoCollectionPost200Response, Error<SapiV1PortfolioRepayFuturesNegativeBalancePostError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_timestamp = timestamp;
    let p_signature = signature;
    let p_recv_window = recv_window;

    let uri_str = format!("{}/sapi/v1/portfolio/repay-futures-negative-balance", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref param_value) = p_recv_window {
        req_builder = req_builder.query(&[("recvWindow", &param_value.to_string())]);
    }
    req_builder = req_builder.query(&[("timestamp", &p_timestamp.to_string())]);
    req_builder = req_builder.query(&[("signature", &p_signature.to_string())]);
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("X-MBX-APIKEY", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text().await?;
        let entity: Option<SapiV1PortfolioRepayFuturesNegativeBalancePostError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Query Auto-repay-futures Status  Weight(IP): 30
pub async fn sapi_v1_portfolio_repay_futures_switch_get(configuration: &configuration::Configuration, timestamp: i64, signature: &str, recv_window: Option<i64>) -> Result<models::SapiV1PortfolioRepayFuturesSwitchGet200Response, Error<SapiV1PortfolioRepayFuturesSwitchGetError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_timestamp = timestamp;
    let p_signature = signature;
    let p_recv_window = recv_window;

    let uri_str = format!("{}/sapi/v1/portfolio/repay-futures-switch", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_recv_window {
        req_builder = req_builder.query(&[("recvWindow", &param_value.to_string())]);
    }
    req_builder = req_builder.query(&[("timestamp", &p_timestamp.to_string())]);
    req_builder = req_builder.query(&[("signature", &p_signature.to_string())]);
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("X-MBX-APIKEY", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text().await?;
        let entity: Option<SapiV1PortfolioRepayFuturesSwitchGetError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Change Auto-repay-futures Status  Weight(IP): 1500
pub async fn sapi_v1_portfolio_repay_futures_switch_post(configuration: &configuration::Configuration, auto_repay: bool, timestamp: i64, signature: &str, recv_window: Option<i64>) -> Result<models::SapiV1PortfolioAutoCollectionPost200Response, Error<SapiV1PortfolioRepayFuturesSwitchPostError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_auto_repay = auto_repay;
    let p_timestamp = timestamp;
    let p_signature = signature;
    let p_recv_window = recv_window;

    let uri_str = format!("{}/sapi/v1/portfolio/repay-futures-switch", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    req_builder = req_builder.query(&[("autoRepay", &p_auto_repay.to_string())]);
    if let Some(ref param_value) = p_recv_window {
        req_builder = req_builder.query(&[("recvWindow", &param_value.to_string())]);
    }
    req_builder = req_builder.query(&[("timestamp", &p_timestamp.to_string())]);
    req_builder = req_builder.query(&[("signature", &p_signature.to_string())]);
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("X-MBX-APIKEY", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text().await?;
        let entity: Option<SapiV1PortfolioRepayFuturesSwitchPostError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Repay Portfolio Margin Bankruptcy Loan.  Weight(UID): 3000
pub async fn sapi_v1_portfolio_repay_post(configuration: &configuration::Configuration, timestamp: i64, signature: &str, from: Option<&str>, recv_window: Option<i64>) -> Result<models::SapiV1PortfolioRepayPost200Response, Error<SapiV1PortfolioRepayPostError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_timestamp = timestamp;
    let p_signature = signature;
    let p_from = from;
    let p_recv_window = recv_window;

    let uri_str = format!("{}/sapi/v1/portfolio/repay", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref param_value) = p_from {
        req_builder = req_builder.query(&[("from", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_recv_window {
        req_builder = req_builder.query(&[("recvWindow", &param_value.to_string())]);
    }
    req_builder = req_builder.query(&[("timestamp", &p_timestamp.to_string())]);
    req_builder = req_builder.query(&[("signature", &p_signature.to_string())]);
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("X-MBX-APIKEY", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text().await?;
        let entity: Option<SapiV1PortfolioRepayPostError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

