use crate::model::*;
use serde::{Deserialize, Serialize};
use tokio::net::TcpStream;
use tokio_tungstenite::tungstenite::Result;
use tokio_tungstenite::{connect_async, MaybeTlsStream};
use typed_websocket::TypedWebSocketStream;

pub type Stream<INPUT, OUTPUT> = TypedWebSocketStream<MaybeTlsStream<TcpStream>, INPUT, OUTPUT>;



#[derive(Debug)]
pub struct BitwyrePrivateEndpointsClient {
	base_url: String,
}

impl BitwyrePrivateEndpointsClient {
	/// connect to the bitwyre websocket server
	pub async fn new() -> Self {
		Self {
			base_url: "wss://api.bitwyre.com/ws/private".to_string(),
 		}
	}
/// This helps in approximating the time difference between the server and client.  
/// receive: TimeMessagePayload  
/// ```
/// #[tokio::main]
/// async fn main() {
///     use exchange_collection_ws_bitwyre:: client::private_endpoints::BitwyrePrivateEndpoints Client;
///     let mut client = BitwyrePrivateEndpoints Client::new ().await;
///     let mut stream = client.time().await.expect("failed connecting websocket stream");
///     let message = stream.receive().await.expect("failed receiving message");
///     println!("{:?}", message);
/// }
/// ```
pub async fn time(&mut self) -> Result<Stream<(), TimeMessagePayload>> {
	let endpooint_url = format!("{}{}", self.base_url, "/time");

	let(ws_stream, _) = connect_async(endpooint_url).await.map_err(| err | {
		eprintln!("Failed to connect: {:?}", err);
		err
	}) ?;

	Ok(TypedWebSocketStream::new(ws_stream))
}

/// To subscribe to topics, send them as a comma-separated list in your connection string.  
/// send: AnonymousSchema_12  
/// receive: StreamResponse  
pub async fn stream(&mut self) -> Result<Stream<AnonymousSchema_12, StreamResponse>> {
	let endpooint_url = format!("{}{}", self.base_url, "/stream");

	let(ws_stream, _) = connect_async(endpooint_url).await.map_err(| err | {
		eprintln!("Failed to connect: {:?}", err);
		err
	}) ?;

	Ok(TypedWebSocketStream::new(ws_stream))
}

/// Retrieve all user's fiat and crypto balance.  
/// send: AccountBalanceSubscribeRequest  
/// receive: AccountBalanceMessage  
pub async fn account_balance(&mut self) -> Result<Stream<AccountBalanceSubscribeRequest, AccountBalanceMessage>> {
	let endpooint_url = format!("{}{}", self.base_url, "/account/balance");

	let(ws_stream, _) = connect_async(endpooint_url).await.map_err(| err | {
		eprintln!("Failed to connect: {:?}", err);
		err
	}) ?;

	Ok(TypedWebSocketStream::new(ws_stream))
}

/// Retrieve user's withdraw and deposit history.  
/// send: AccountBalanceSubscribeRequest  
/// receive: AccountStatement  
pub async fn account_statement(&mut self) -> Result<Stream<AccountBalanceSubscribeRequest, AccountStatement>> {
	let endpooint_url = format!("{}{}", self.base_url, "/account/statement");

	let(ws_stream, _) = connect_async(endpooint_url).await.map_err(| err | {
		eprintln!("Failed to connect: {:?}", err);
		err
	}) ?;

	Ok(TypedWebSocketStream::new(ws_stream))
}

/// Create Order Request  
/// send: AccountBalanceSubscribeRequest  
/// receive: OrderMessage  
pub async fn orders_control(&mut self) -> Result<Stream<AccountBalanceSubscribeRequest, OrderMessage>> {
	let endpooint_url = format!("{}{}", self.base_url, "/orders/control");

	let(ws_stream, _) = connect_async(endpooint_url).await.map_err(| err | {
		eprintln!("Failed to connect: {:?}", err);
		err
	}) ?;

	Ok(TypedWebSocketStream::new(ws_stream))
}

/// get order status  
/// send: AccountBalanceSubscribeRequest  
/// receive: OrderMessage  
pub async fn orders_status(&mut self) -> Result<Stream<AccountBalanceSubscribeRequest, OrderMessage>> {
	let endpooint_url = format!("{}{}", self.base_url, "/orders/status");

	let(ws_stream, _) = connect_async(endpooint_url).await.map_err(| err | {
		eprintln!("Failed to connect: {:?}", err);
		err
	}) ?;

	Ok(TypedWebSocketStream::new(ws_stream))
}

/// Subscribe to eventual broadcasts on order status changes, can only be filtered by instrument/Asset Pair symbol.  
/// send: AccountBalanceSubscribeRequest  
/// receive: OrderMessage  
pub async fn orders_events(&mut self) -> Result<Stream<AccountBalanceSubscribeRequest, OrderMessage>> {
	let endpooint_url = format!("{}{}", self.base_url, "/orders/events");

	let(ws_stream, _) = connect_async(endpooint_url).await.map_err(| err | {
		eprintln!("Failed to connect: {:?}", err);
		err
	}) ?;

	Ok(TypedWebSocketStream::new(ws_stream))
}

/// Retrieves some or all latest buying/selling transaction histories.  
/// send: AccountBalanceSubscribeRequest  
/// receive: TradeHistoryMessage  
pub async fn trades_history(&mut self) -> Result<Stream<AccountBalanceSubscribeRequest, TradeHistoryMessage>> {
	let endpooint_url = format!("{}{}", self.base_url, "/trades/history");

	let(ws_stream, _) = connect_async(endpooint_url).await.map_err(| err | {
		eprintln!("Failed to connect: {:?}", err);
		err
	}) ?;

	Ok(TypedWebSocketStream::new(ws_stream))
}

/// Retrieve all user's chat message encrypted in Elliptic Curve Integrated Encryption Scheme (ECIES).  
/// send: AccountBalanceSubscribeRequest  
/// receive: ChatHistory  
pub async fn chat(&mut self) -> Result<Stream<AccountBalanceSubscribeRequest, ChatHistory>> {
	let endpooint_url = format!("{}{}", self.base_url, "/chat/");

	let(ws_stream, _) = connect_async(endpooint_url).await.map_err(| err | {
		eprintln!("Failed to connect: {:?}", err);
		err
	}) ?;

	Ok(TypedWebSocketStream::new(ws_stream))
}

/// send chat  
/// send: AccountBalanceSubscribeRequest  
/// receive: SentMessage  
pub async fn chats(&mut self) -> Result<Stream<AccountBalanceSubscribeRequest, SentMessage>> {
	let endpooint_url = format!("{}{}", self.base_url, "/chats");

	let(ws_stream, _) = connect_async(endpooint_url).await.map_err(| err | {
		eprintln!("Failed to connect: {:?}", err);
		err
	}) ?;

	Ok(TypedWebSocketStream::new(ws_stream))
}

/// get notifications  
/// send: AccountBalanceSubscribeRequest  
/// receive: Notification  
pub async fn notifications(&mut self) -> Result<Stream<AccountBalanceSubscribeRequest, Notification>> {
	let endpooint_url = format!("{}{}", self.base_url, "/notifications");

	let(ws_stream, _) = connect_async(endpooint_url).await.map_err(| err | {
		eprintln!("Failed to connect: {:?}", err);
		err
	}) ?;

	Ok(TypedWebSocketStream::new(ws_stream))
}
}
