use crate::model::*;
use serde::{Deserialize, Serialize};
use tokio::net::TcpStream;
use tokio_tungstenite::tungstenite::Result;
use tokio_tungstenite::{connect_async, MaybeTlsStream};
use typed_websocket::TypedWebSocketStream;

pub type Stream<INPUT, OUTPUT> = TypedWebSocketStream<MaybeTlsStream<TcpStream>, INPUT, OUTPUT>;



#[derive(Debug)]
pub struct BinanceDevelopmentClient {
	base_url: String,
}

impl BinanceDevelopmentClient {
	/// connect to the binance websocket server
	pub async fn new() -> Self {
		Self {
			base_url: "wss://stream.binance.com:443/ws".to_string(),
 		}
	}
/// Channel for managing data streams, including subscribing, unsubscribing, listing subscriptions, setting properties, and handling errors.
  
/// send: StreamControlMessagePayload  
/// receive: StreamControlResponse  
pub async fn stream(&mut self) -> Result<Stream<StreamControlMessagePayload, StreamControlResponse>> {
	let endpooint_url = format!("{}{}", self.base_url, "/stream");

	let(ws_stream, _) = connect_async(endpooint_url).await.map_err(| err | {
		eprintln!("Failed to connect: {:?}", err);
		err
	}) ?;

	Ok(TypedWebSocketStream::new(ws_stream))
}
}
