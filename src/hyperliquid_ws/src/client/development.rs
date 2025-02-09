use crate::model::*;
use serde::{Deserialize, Serialize};
use tokio::net::TcpStream;
use tokio_tungstenite::tungstenite::Result;
use tokio_tungstenite::{connect_async, MaybeTlsStream};
use typed_websocket::TypedWebSocketStream;

pub type Stream<INPUT, OUTPUT> = TypedWebSocketStream<MaybeTlsStream<TcpStream>, INPUT, OUTPUT>;

/// union response for channel: Subscribe
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum SubscribeEnumResponse {
	}

#[derive(Debug)]
pub struct HyperliquidDevelopmentClient {
	base_url: String,
}

impl HyperliquidDevelopmentClient {
	/// connect to the hyperliquid websocket server
	pub async fn new() -> Self {
		Self {
			base_url: "wss://api.hyperliquid-testnet.xyz/ws".to_string(),
 		}
	}
/// Subscription channel for receiving real-time data.  
/// send: SubscriptionPayload  
/// receive: SubscribeEnumResponse
pub async fn subscribe(& mut self) -> Result<Stream<SubscriptionPayload, SubscribeEnumResponse>> {
		let endpooint_url = format!("{}{}", self.base_url, "subscribe");

		let(ws_stream, _) = connect_async(endpooint_url).await.map_err(| err | {
			eprintln!("Failed to connect: {:?}", err);
		err
		}) ?;

		Ok(TypedWebSocketStream::new(ws_stream))
}
}
