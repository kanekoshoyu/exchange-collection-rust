use crate::model::*;
use serde::{Deserialize, Serialize};
use tokio::net::TcpStream;
use tokio_tungstenite::tungstenite::Result;
use tokio_tungstenite::{connect_async, MaybeTlsStream};
use typed_websocket::TypedWebSocketStream;

pub type Stream<INPUT, OUTPUT> = TypedWebSocketStream<MaybeTlsStream<TcpStream>, INPUT, OUTPUT>;

/// union response for channel: SymbolTrades
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum SymbolTradesEnumResponse {
		/// message: TradeData, payload: TradeData
	TradeData(TradeData),
	/// message: SubscribeResponse, payload: SubscribeResponse
	SubscribeResponse(SubscribeResponse),
}

/// union response for channel: SymbolKline
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum SymbolKlineEnumResponse {
		/// message: KlineData, payload: KlineData
	KlineData(KlineData),
	/// message: SubscribeResponse, payload: SubscribeResponse
	SubscribeResponse(SubscribeResponse),
}

/// union response for channel: SymbolDepth
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum SymbolDepthEnumResponse {
		/// message: DepthData, payload: DepthData
	DepthData(DepthData),
	/// message: SubscribeResponse, payload: SubscribeResponse
	SubscribeResponse(SubscribeResponse),
}

/// union response for channel: SymbolPartialDepth
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum SymbolPartialDepthEnumResponse {
		/// message: PartialDepthData, payload: PartialDepthData
	PartialDepthData(PartialDepthData),
	/// message: SubscribeResponse, payload: SubscribeResponse
	SubscribeResponse(SubscribeResponse),
}

/// union response for channel: SymbolBookTicker
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum SymbolBookTickerEnumResponse {
		/// message: BookTickerData, payload: BookTickerData
	BookTickerData(BookTickerData),
	/// message: SubscribeResponse, payload: SubscribeResponse
	SubscribeResponse(SubscribeResponse),
}

/// union response for channel: SymbolMiniTicker
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum SymbolMiniTickerEnumResponse {
		/// message: MiniTickerData, payload: MiniTickerData
	MiniTickerData(MiniTickerData),
	/// message: SubscribeResponse, payload: SubscribeResponse
	SubscribeResponse(SubscribeResponse),
}

/// union response for channel: MiniTickers
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum MiniTickersEnumResponse {
		/// message: MiniTickersData, payload: MiniTickersData
	MiniTickersData(MiniTickersData),
	/// message: SubscribeResponse, payload: SubscribeResponse
	SubscribeResponse(SubscribeResponse),
}

/// union response for channel: AccountUpdates
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum AccountUpdatesEnumResponse {
		/// message: AccountUpdateData, payload: AccountUpdateData
	AccountUpdateData(AccountUpdateData),
	/// message: SubscribeResponse, payload: SubscribeResponse
	SubscribeResponse(SubscribeResponse),
}

/// union response for channel: AccountDeals
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum AccountDealsEnumResponse {
		/// message: AccountDealData, payload: AccountDealData
	AccountDealData(AccountDealData),
	/// message: SubscribeResponse, payload: SubscribeResponse
	SubscribeResponse(SubscribeResponse),
}

/// union response for channel: AccountOrders
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum AccountOrdersEnumResponse {
		/// message: AccountOrderData, payload: AccountOrderData
	AccountOrderData(AccountOrderData),
	/// message: SubscribeResponse, payload: SubscribeResponse
	SubscribeResponse(SubscribeResponse),
}

#[derive(Debug)]
pub struct MexcWebsocketEndpointsClient {
	base_url: String,
}

impl MexcWebsocketEndpointsClient {
	/// connect to the mexc websocket server
	pub async fn new() -> Self {
		Self {
			base_url: "wss://wbs.mexc.com/ws/".to_string(),
 		}
	}
/// Channel for subscribing, unsubscribing, and pinging.
pub async fn stream_control(&self) {
	todo!("unmapped case, check the logic and implement this case, send=3, receive=3");
}

/// Trade stream for a specific symbol like BTCUSDT etc  
/// send: SubscribeRequest  
/// receive: SymbolTradesEnumResponse
pub async fn symbol_trades(& mut self) -> Result<Stream<SubscribeRequest, SymbolTradesEnumResponse>> {
		let endpooint_url = format!("{}{}", self.base_url, "{symbol}-trades");

		let(ws_stream, _) = connect_async(endpooint_url).await.map_err(| err | {
			eprintln!("Failed to connect: {:?}", err);
		err
		}) ?;

		Ok(TypedWebSocketStream::new(ws_stream))
}

/// Kline/Candlestick stream for a specific symbol (e.g., BTCUSDT) and interval (e.g., Min15, Hour4, Day1).  
/// send: SubscribeRequest  
/// receive: SymbolKlineEnumResponse
pub async fn symbol_kline(& mut self) -> Result<Stream<SubscribeRequest, SymbolKlineEnumResponse>> {
		let endpooint_url = format!("{}{}", self.base_url, "{symbol}-kline");

		let(ws_stream, _) = connect_async(endpooint_url).await.map_err(| err | {
			eprintln!("Failed to connect: {:?}", err);
		err
		}) ?;

		Ok(TypedWebSocketStream::new(ws_stream))
}

/// Incremental depth stream for a specific symbol e.g; BTCUSDT.  
/// send: SubscribeRequest  
/// receive: SymbolDepthEnumResponse
pub async fn symbol_depth(& mut self) -> Result<Stream<SubscribeRequest, SymbolDepthEnumResponse>> {
		let endpooint_url = format!("{}{}", self.base_url, "{symbol}-depth");

		let(ws_stream, _) = connect_async(endpooint_url).await.map_err(| err | {
			eprintln!("Failed to connect: {:?}", err);
		err
		}) ?;

		Ok(TypedWebSocketStream::new(ws_stream))
}

/// Partial book depth stream (top bids and asks) for a specific symbol (e.g., BTCUSDT) and level (5, 10, or 20).  
/// send: SubscribeRequest  
/// receive: SymbolPartialDepthEnumResponse
pub async fn symbol_partial_depth(& mut self) -> Result<Stream<SubscribeRequest, SymbolPartialDepthEnumResponse>> {
		let endpooint_url = format!("{}{}", self.base_url, "{symbol}-partial-depth");

		let(ws_stream, _) = connect_async(endpooint_url).await.map_err(| err | {
			eprintln!("Failed to connect: {:?}", err);
		err
		}) ?;

		Ok(TypedWebSocketStream::new(ws_stream))
}

/// Individual symbol book ticker stream.  
/// send: SubscribeRequest  
/// receive: SymbolBookTickerEnumResponse
pub async fn symbol_book_ticker(& mut self) -> Result<Stream<SubscribeRequest, SymbolBookTickerEnumResponse>> {
		let endpooint_url = format!("{}{}", self.base_url, "{symbol}-book-ticker");

		let(ws_stream, _) = connect_async(endpooint_url).await.map_err(| err | {
			eprintln!("Failed to connect: {:?}", err);
		err
		}) ?;

		Ok(TypedWebSocketStream::new(ws_stream))
}

/// Mini-ticker stream for a specific symbol and timezone(e.g., UTC+8).  
/// send: SubscribeRequest  
/// receive: SymbolMiniTickerEnumResponse
pub async fn symbol_mini_ticker(& mut self) -> Result<Stream<SubscribeRequest, SymbolMiniTickerEnumResponse>> {
		let endpooint_url = format!("{}{}", self.base_url, "{symbol}-mini-ticker");

		let(ws_stream, _) = connect_async(endpooint_url).await.map_err(| err | {
			eprintln!("Failed to connect: {:?}", err);
		err
		}) ?;

		Ok(TypedWebSocketStream::new(ws_stream))
}

/// Mini-tickers stream for all symbols in a specific timezone.  
/// send: SubscribeRequest  
/// receive: MiniTickersEnumResponse
pub async fn mini_tickers(& mut self) -> Result<Stream<SubscribeRequest, MiniTickersEnumResponse>> {
		let endpooint_url = format!("{}{}", self.base_url, "mini-tickers");

		let(ws_stream, _) = connect_async(endpooint_url).await.map_err(| err | {
			eprintln!("Failed to connect: {:?}", err);
		err
		}) ?;

		Ok(TypedWebSocketStream::new(ws_stream))
}

/// Spot account update stream.  
/// send: SubscribeRequest  
/// receive: AccountUpdatesEnumResponse
pub async fn account_updates(& mut self) -> Result<Stream<SubscribeRequest, AccountUpdatesEnumResponse>> {
		let endpooint_url = format!("{}{}", self.base_url, "account-updates");

		let(ws_stream, _) = connect_async(endpooint_url).await.map_err(| err | {
			eprintln!("Failed to connect: {:?}", err);
		err
		}) ?;

		Ok(TypedWebSocketStream::new(ws_stream))
}

/// Spot account deals stream.  
/// send: SubscribeRequest  
/// receive: AccountDealsEnumResponse
pub async fn account_deals(& mut self) -> Result<Stream<SubscribeRequest, AccountDealsEnumResponse>> {
		let endpooint_url = format!("{}{}", self.base_url, "account-deals");

		let(ws_stream, _) = connect_async(endpooint_url).await.map_err(| err | {
			eprintln!("Failed to connect: {:?}", err);
		err
		}) ?;

		Ok(TypedWebSocketStream::new(ws_stream))
}

/// Spot account orders stream.  
/// send: SubscribeRequest  
/// receive: AccountOrdersEnumResponse
pub async fn account_orders(& mut self) -> Result<Stream<SubscribeRequest, AccountOrdersEnumResponse>> {
		let endpooint_url = format!("{}{}", self.base_url, "account-orders");

		let(ws_stream, _) = connect_async(endpooint_url).await.map_err(| err | {
			eprintln!("Failed to connect: {:?}", err);
		err
		}) ?;

		Ok(TypedWebSocketStream::new(ws_stream))
}
}
