use crate::model::*;
use serde::{Deserialize, Serialize};
use tokio::net::TcpStream;
use tokio_tungstenite::tungstenite::Result;
use tokio_tungstenite::{connect_async, MaybeTlsStream};
use typed_websocket::TypedWebSocketStream;

pub type Stream<INPUT, OUTPUT> = TypedWebSocketStream<MaybeTlsStream<TcpStream>, INPUT, OUTPUT>;



#[derive(Debug)]
pub struct CoincheckWebsocketEndpointsClient {
	base_url: String,
}

impl CoincheckWebsocketEndpointsClient {
	/// connect to the coincheck websocket server
	pub async fn new() -> Self {
		Self {
			base_url: "wss://ws-api.coincheck.com/".to_string(),
 		}
	}
/// The Public Trades Channel pushes information on recent transactions every 0.1 seconds of specific pair like btc_jpy  
/// send: Request  
/// receive: TradeData  
pub async fn pair_trades(&mut self) -> Result<Stream<Request, TradeData>> {
	let endpooint_url = format!("{}{}", self.base_url, "{pair}-trades");

	let(ws_stream, _) = connect_async(endpooint_url).await.map_err(| err | {
		eprintln!("Failed to connect: {:?}", err);
		err
	}) ?;

	Ok(TypedWebSocketStream::new(ws_stream))
}

/// Send the difference of order book information at regular intervals.  
/// send: Request  
/// receive: OrderbookData  
pub async fn pair_orderbook(&mut self) -> Result<Stream<Request, OrderbookData>> {
	let endpooint_url = format!("{}{}", self.base_url, "{pair}-orderbook");

	let(ws_stream, _) = connect_async(endpooint_url).await.map_err(| err | {
		eprintln!("Failed to connect: {:?}", err);
		err
	}) ?;

	Ok(TypedWebSocketStream::new(ws_stream))
}
}
