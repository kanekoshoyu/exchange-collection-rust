use crate::model::*;
use serde::{Deserialize, Serialize};
use tokio::net::TcpStream;
use tokio_tungstenite::tungstenite::Result;
use tokio_tungstenite::{connect_async, MaybeTlsStream};
use typed_websocket::TypedWebSocketStream;

pub type Stream<INPUT, OUTPUT> = TypedWebSocketStream<MaybeTlsStream<TcpStream>, INPUT, OUTPUT>;



#[derive(Debug)]
pub struct HashkeyProductionPublicStreamsClient {
	base_url: String,
}

impl HashkeyProductionPublicStreamsClient {
	/// connect to the hashkey websocket server
	pub async fn new() -> Self {
		Self {
			base_url: "wss://stream-pro.hashkey.com/api/v1/ws/{listenKey}".to_string(),
 		}
	}
/// Main communication channel for the WebSocket connection.  
/// send: PongPayload  
/// receive: AnonymousSchema_1  
pub async fn (&mut self) -> Result<Stream<PongPayload, AnonymousSchema_1>> {
	let endpooint_url = format!("{}{}", self.base_url, "/");

	let(ws_stream, _) = connect_async(endpooint_url).await.map_err(| err | {
		eprintln!("Failed to connect: {:?}", err);
		err
	}) ?;

	Ok(TypedWebSocketStream::new(ws_stream))
}
}
