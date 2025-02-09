use crate::model::*;
use serde::{Deserialize, Serialize};
use tokio::net::TcpStream;
use tokio_tungstenite::tungstenite::Result;
use tokio_tungstenite::{connect_async, MaybeTlsStream};
use typed_websocket::TypedWebSocketStream;

pub type Stream<INPUT, OUTPUT> = TypedWebSocketStream<MaybeTlsStream<TcpStream>, INPUT, OUTPUT>;



#[derive(Debug)]
pub struct BitgetPrivateClient {
	base_url: String,
}

impl BitgetPrivateClient {
	/// connect to the bitget websocket server
	pub async fn new() -> Self {
		Self {
			base_url: "wss://ws.bitget.com/v2/ws/private".to_string(),
 		}
	}
/// Get the product's latest price, bid price, bid price and 24h trading volume information. Frequency of data push: 100ms ~ 300ms
  
/// send: Payload  
/// receive: PublishedMessage  
pub async fn ticker(&mut self) -> Result<Stream<Payload, PublishedMessage>> {
	let endpooint_url = format!("{}{}", self.base_url, "ticker");

	let(ws_stream, _) = connect_async(endpooint_url).await.map_err(| err | {
		eprintln!("Failed to connect: {:?}", err);
		err
	}) ?;

	Ok(TypedWebSocketStream::new(ws_stream))
}

/// Get the candlestick data of the product.After first subscription, it will push the recent snapshot data and then push the update data
  
/// send: Payload  
/// receive: PublishedMessage  
pub async fn candlestick(&mut self) -> Result<Stream<Payload, PublishedMessage>> {
	let endpooint_url = format!("{}{}", self.base_url, "candlestick");

	let(ws_stream, _) = connect_async(endpooint_url).await.map_err(| err | {
		eprintln!("Failed to connect: {:?}", err);
		err
	}) ?;

	Ok(TypedWebSocketStream::new(ws_stream))
}

/// Push once if any trade is matched(taker orders).After first subscription, it will push the recent snapshot data and then push the update data
  
/// send: Payload  
/// receive: PublishedMessage  
pub async fn trade(&mut self) -> Result<Stream<Payload, PublishedMessage>> {
	let endpooint_url = format!("{}{}", self.base_url, "trade");

	let(ws_stream, _) = connect_async(endpooint_url).await.map_err(| err | {
		eprintln!("Failed to connect: {:?}", err);
		err
	}) ?;

	Ok(TypedWebSocketStream::new(ws_stream))
}

/// Use books for full data, book1 for 1 depth level, book5 for 5 depth levels, book15 for 15 depth levels.Default data push frequency is 200ms.Default data push frequency for books1 is 60ms
  
/// send: Payload  
/// receive: PublishedMessage  
pub async fn depth(&mut self) -> Result<Stream<Payload, PublishedMessage>> {
	let endpooint_url = format!("{}{}", self.base_url, "depth");

	let(ws_stream, _) = connect_async(endpooint_url).await.map_err(| err | {
		eprintln!("Failed to connect: {:?}", err);
		err
	}) ?;

	Ok(TypedWebSocketStream::new(ws_stream))
}

/// Trade details channels
  
/// send: Payload  
/// receive: PublishedMessage  
pub async fn fill(&mut self) -> Result<Stream<Payload, PublishedMessage>> {
	let endpooint_url = format!("{}{}", self.base_url, "fill");

	let(ws_stream, _) = connect_async(endpooint_url).await.map_err(| err | {
		eprintln!("Failed to connect: {:?}", err);
		err
	}) ?;

	Ok(TypedWebSocketStream::new(ws_stream))
}

/// Get order information. Initial subscriptions will not trigger any push notifications.Data will be pushed when the following events occurred:<br><ul><li>Place orders</li><li>Orders are filled</li>Cancel orders</li><li>Modify orders</li></ul>
  
/// send: Payload  
/// receive: PublishedMessage  
pub async fn orders(&mut self) -> Result<Stream<Payload, PublishedMessage>> {
	let endpooint_url = format!("{}{}", self.base_url, "orders");

	let(ws_stream, _) = connect_async(endpooint_url).await.map_err(| err | {
		eprintln!("Failed to connect: {:?}", err);
		err
	}) ?;

	Ok(TypedWebSocketStream::new(ws_stream))
}

/// Subscribe trigger order channel
  
/// send: Payload  
/// receive: PublishedMessage  
pub async fn trigger(&mut self) -> Result<Stream<Payload, PublishedMessage>> {
	let endpooint_url = format!("{}{}", self.base_url, "trigger");

	let(ws_stream, _) = connect_async(endpooint_url).await.map_err(| err | {
		eprintln!("Failed to connect: {:?}", err);
		err
	}) ?;

	Ok(TypedWebSocketStream::new(ws_stream))
}

/// Get account information, push data according to the subscription dimensions for the first subscription.Data will be pushed when the following events occurred:<br><ul><li>Orders are filled</li><li>Transfer</li><li>Deposit</li><li>Withdrawal</li></ul>
  
/// send: Payload  
/// receive: PublishedMessage  
pub async fn account(&mut self) -> Result<Stream<Payload, PublishedMessage>> {
	let endpooint_url = format!("{}{}", self.base_url, "account");

	let(ws_stream, _) = connect_async(endpooint_url).await.map_err(| err | {
		eprintln!("Failed to connect: {:?}", err);
		err
	}) ?;

	Ok(TypedWebSocketStream::new(ws_stream))
}
}
