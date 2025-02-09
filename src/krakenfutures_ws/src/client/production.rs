use crate::model::*;
use serde::{Deserialize, Serialize};
use tokio::net::TcpStream;
use tokio_tungstenite::tungstenite::Result;
use tokio_tungstenite::{connect_async, MaybeTlsStream};
use typed_websocket::TypedWebSocketStream;

pub type Stream<INPUT, OUTPUT> = TypedWebSocketStream<MaybeTlsStream<TcpStream>, INPUT, OUTPUT>;



#[derive(Debug)]
pub struct KrakenfuturesProductionClient {
	base_url: String,
}

impl KrakenfuturesProductionClient {
	/// connect to the krakenfutures websocket server
	pub async fn new() -> Self {
		Self {
			base_url: "wss://futures.kraken.com/ws/v1".to_string(),
 		}
	}
/// This subscription feed publishes information about user open orders.
  
/// send: OpenOrdersRequest  
/// receive: OpenOrdersSnapshot  
pub async fn open_orders(&mut self) -> Result<Stream<OpenOrdersRequest, OpenOrdersSnapshot>> {
	let endpooint_url = format!("{}{}", self.base_url, "open_orders");

	let(ws_stream, _) = connect_async(endpooint_url).await.map_err(| err | {
		eprintln!("Failed to connect: {:?}", err);
		err
	}) ?;

	Ok(TypedWebSocketStream::new(ws_stream))
}

/// This subscription feed publishes information about user open orders. This feed adds extra information about all the post-only orders that failed to cross the book.
  
/// send: OpenOrdersRequest  
/// receive: OpenOrdersSnapshot  
pub async fn open_orders_verbose(&mut self) -> Result<Stream<OpenOrdersRequest, OpenOrdersSnapshot>> {
	let endpooint_url = format!("{}{}", self.base_url, "open_orders_verbose");

	let(ws_stream, _) = connect_async(endpooint_url).await.map_err(| err | {
		eprintln!("Failed to connect: {:?}", err);
		err
	}) ?;

	Ok(TypedWebSocketStream::new(ws_stream))
}

/// This subscription feed publishes fills information.
  
/// send: OpenOrdersRequest  
/// receive: FillsResponse  
pub async fn fills(&mut self) -> Result<Stream<OpenOrdersRequest, FillsResponse>> {
	let endpooint_url = format!("{}{}", self.base_url, "fills");

	let(ws_stream, _) = connect_async(endpooint_url).await.map_err(| err | {
		eprintln!("Failed to connect: {:?}", err);
		err
	}) ?;

	Ok(TypedWebSocketStream::new(ws_stream))
}

/// This feed returns balance information for holding wallets, single collateral wallets and multi-collateral wallets.
  
/// send: OpenOrdersRequest  
/// receive: OpenOrdersResponse  
pub async fn balances(&mut self) -> Result<Stream<OpenOrdersRequest, OpenOrdersResponse>> {
	let endpooint_url = format!("{}{}", self.base_url, "balances");

	let(ws_stream, _) = connect_async(endpooint_url).await.map_err(| err | {
		eprintln!("Failed to connect: {:?}", err);
		err
	}) ?;

	Ok(TypedWebSocketStream::new(ws_stream))
}

/// This subscription feed publishes the open positions of the user account.
  
/// send: OpenOrdersRequest  
/// receive: OpenOrdersResponse  
pub async fn open_positions(&mut self) -> Result<Stream<OpenOrdersRequest, OpenOrdersResponse>> {
	let endpooint_url = format!("{}{}", self.base_url, "open_positions");

	let(ws_stream, _) = connect_async(endpooint_url).await.map_err(| err | {
		eprintln!("Failed to connect: {:?}", err);
		err
	}) ?;

	Ok(TypedWebSocketStream::new(ws_stream))
}

/// This subscription feed publishes account log.
  
/// send: OpenOrdersRequest  
/// receive: OpenOrdersResponse  
pub async fn account_log(&mut self) -> Result<Stream<OpenOrdersRequest, OpenOrdersResponse>> {
	let endpooint_url = format!("{}{}", self.base_url, "account_log");

	let(ws_stream, _) = connect_async(endpooint_url).await.map_err(| err | {
		eprintln!("Failed to connect: {:?}", err);
		err
	}) ?;

	Ok(TypedWebSocketStream::new(ws_stream))
}

/// This subscription feed publishes notifications to the client.Authentication is required.
  
/// send: OpenOrdersRequest  
/// receive: OpenOrdersResponse  
pub async fn notifications_auth(&mut self) -> Result<Stream<OpenOrdersRequest, OpenOrdersResponse>> {
	let endpooint_url = format!("{}{}", self.base_url, "notifications_auth");

	let(ws_stream, _) = connect_async(endpooint_url).await.map_err(| err | {
		eprintln!("Failed to connect: {:?}", err);
		err
	}) ?;

	Ok(TypedWebSocketStream::new(ws_stream))
}

/// The ticker feed returns ticker information about listed products. Only tradeable markets are available via individual WebSocket market data feeds. Delta messages are throttled such that they are published every 1s.
  
/// send: TickerRequest  
/// receive: TickerResponse  
pub async fn ticker(&mut self) -> Result<Stream<TickerRequest, TickerResponse>> {
	let endpooint_url = format!("{}{}", self.base_url, "ticker");

	let(ws_stream, _) = connect_async(endpooint_url).await.map_err(| err | {
		eprintln!("Failed to connect: {:?}", err);
		err
	}) ?;

	Ok(TypedWebSocketStream::new(ws_stream))
}

/// The ticker lite feed returns ticker information about listed products. Delta messages are throttled such that they are published every 1s.
  
/// send: TickerRequest  
/// receive: TickerResponse  
pub async fn ticker_lite(&mut self) -> Result<Stream<TickerRequest, TickerResponse>> {
	let endpooint_url = format!("{}{}", self.base_url, "ticker_lite");

	let(ws_stream, _) = connect_async(endpooint_url).await.map_err(| err | {
		eprintln!("Failed to connect: {:?}", err);
		err
	}) ?;

	Ok(TypedWebSocketStream::new(ws_stream))
}

/// The book feed returns information about the order book.
  
/// send: TickerRequest  
/// receive: TickerResponse  
pub async fn book(&mut self) -> Result<Stream<TickerRequest, TickerResponse>> {
	let endpooint_url = format!("{}{}", self.base_url, "book");

	let(ws_stream, _) = connect_async(endpooint_url).await.map_err(| err | {
		eprintln!("Failed to connect: {:?}", err);
		err
	}) ?;

	Ok(TypedWebSocketStream::new(ws_stream))
}

/// The trade feed returns information about executed trades
  
/// send: TickerRequest  
/// receive: TickerResponse  
pub async fn trade(&mut self) -> Result<Stream<TickerRequest, TickerResponse>> {
	let endpooint_url = format!("{}{}", self.base_url, "trade");

	let(ws_stream, _) = connect_async(endpooint_url).await.map_err(| err | {
		eprintln!("Failed to connect: {:?}", err);
		err
	}) ?;

	Ok(TypedWebSocketStream::new(ws_stream))
}

/// This request returns a challenge to be used in handshake for user authentication.
  
/// send: ChallengeRequest  
/// receive: ChallengeResponse  
pub async fn challenge(&mut self) -> Result<Stream<ChallengeRequest, ChallengeResponse>> {
	let endpooint_url = format!("{}{}", self.base_url, "challenge");

	let(ws_stream, _) = connect_async(endpooint_url).await.map_err(| err | {
		eprintln!("Failed to connect: {:?}", err);
		err
	}) ?;

	Ok(TypedWebSocketStream::new(ws_stream))
}

/// The heartbeat feed publishes a heartbeat message at timed intervals.
  
/// send: HeartbeatRequest  
/// receive: HeartbeatRequest  
pub async fn heartbeat(&mut self) -> Result<Stream<HeartbeatRequest, HeartbeatRequest>> {
	let endpooint_url = format!("{}{}", self.base_url, "heartbeat");

	let(ws_stream, _) = connect_async(endpooint_url).await.map_err(| err | {
		eprintln!("Failed to connect: {:?}", err);
		err
	}) ?;

	Ok(TypedWebSocketStream::new(ws_stream))
}
}
