use crate::model::*;
use serde::{Deserialize, Serialize};
use tokio::net::TcpStream;
use tokio_tungstenite::tungstenite::Result;
use tokio_tungstenite::{connect_async, MaybeTlsStream};
use typed_websocket::TypedWebSocketStream;

pub type Stream<INPUT, OUTPUT> = TypedWebSocketStream<MaybeTlsStream<TcpStream>, INPUT, OUTPUT>;



#[derive(Debug)]
pub struct CoinbaseMarketDataSandboxClient {
	base_url: String,
}

impl CoinbaseMarketDataSandboxClient {
	/// connect to the coinbase websocket server
	pub async fn new() -> Self {
		Self {
			base_url: "wss://ws-feed-public.sandbox.exchange.coinbase.com/".to_string(),
 		}
	}
/// Subscribe to the level2 order book updates for specific product IDs.  
/// send: SubscriptionArray  
/// receive: PublishData  
pub async fn level2(&mut self) -> Result<Stream<SubscriptionArray, PublishData>> {
	let endpooint_url = format!("{}{}", self.base_url, "level2");

	let(ws_stream, _) = connect_async(endpooint_url).await.map_err(| err | {
		eprintln!("Failed to connect: {:?}", err);
		err
	}) ?;

	Ok(TypedWebSocketStream::new(ws_stream))
}

/// Subscribe to heartbeat updates for specific product IDs.  
/// send: SubscriptionArray  
/// receive: PublishData  
pub async fn heartbeat(&mut self) -> Result<Stream<SubscriptionArray, PublishData>> {
	let endpooint_url = format!("{}{}", self.base_url, "heartbeat");

	let(ws_stream, _) = connect_async(endpooint_url).await.map_err(| err | {
		eprintln!("Failed to connect: {:?}", err);
		err
	}) ?;

	Ok(TypedWebSocketStream::new(ws_stream))
}

/// Subscribe to ticker price updates for specific product IDs.  
/// send: SubscriptionArray  
/// receive: PublishData  
pub async fn ticker(&mut self) -> Result<Stream<SubscriptionArray, PublishData>> {
	let endpooint_url = format!("{}{}", self.base_url, "ticker");

	let(ws_stream, _) = connect_async(endpooint_url).await.map_err(| err | {
		eprintln!("Failed to connect: {:?}", err);
		err
	}) ?;

	Ok(TypedWebSocketStream::new(ws_stream))
}
}
