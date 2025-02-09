use crate::model::*;
use serde::{Deserialize, Serialize};
use tokio::net::TcpStream;
use tokio_tungstenite::tungstenite::Result;
use tokio_tungstenite::{connect_async, MaybeTlsStream};
use typed_websocket::TypedWebSocketStream;

pub type Stream<INPUT, OUTPUT> = TypedWebSocketStream<MaybeTlsStream<TcpStream>, INPUT, OUTPUT>;

/// union response for channel: Account
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum AccountEnumResponse {
		/// message: SubscriptionResponse, payload: SubscribeResponse
	SubscriptionResponse(SubscribeResponse),
	/// message: SubscriptionPushData, payload: PushDataResponse
	SubscriptionPushData(PushDataResponse),
	/// message: ErrorResponse, payload: ErrorResponse
	ErrorResponse(ErrorResponse),
}

/// union response for channel: Positions
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum PositionsEnumResponse {
		/// message: PositionResponse, payload: PositionResponse
	PositionResponse(PositionResponse),
	/// message: PositionPushDataResponse, payload: PositionPushDataResponse
	PositionPushDataResponse(PositionPushDataResponse),
	/// message: ErrorResponse, payload: ErrorResponse
	ErrorResponse(ErrorResponse),
}

/// union response for channel: Balances
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum BalancesEnumResponse {
		/// message: BalanceResponse, payload: BalanceResponse
	BalanceResponse(BalanceResponse),
	/// message: BalancePushDataResponse, payload: BalancePushDataResponse
	BalancePushDataResponse(BalancePushDataResponse),
	/// message: ErrorResponse, payload: ErrorResponse
	ErrorResponse(ErrorResponse),
}

/// union response for channel: PositionRisk
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum PositionRiskEnumResponse {
		/// message: PositionResponse, payload: RiskResponse
	PositionResponse(RiskResponse),
	/// message: PositionPushDataResponse, payload: RiskPushDataResponse
	PositionPushDataResponse(RiskPushDataResponse),
	/// message: ErrorResponse, payload: ErrorResponse
	ErrorResponse(ErrorResponse),
}

/// union response for channel: Order
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum OrderEnumResponse {
		/// message: OrderResponse, payload: OrderResponse
	OrderResponse(OrderResponse),
	/// message: OrderPushDataResponse, payload: OrderPushDataResponse
	OrderPushDataResponse(OrderPushDataResponse),
	/// message: ErrorResponse, payload: ErrorResponse
	ErrorResponse(ErrorResponse),
}

/// union response for channel: Fills
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum FillsEnumResponse {
		/// message: FillsResponse, payload: FillsResponse
	FillsResponse(FillsResponse),
	/// message: FillsPushDataResponse, payload: FillsPushDataResponse
	FillsPushDataResponse(FillsPushDataResponse),
	/// message: ErrorResponse, payload: ErrorResponse
	ErrorResponse(ErrorResponse),
}

/// union response for channel: PlaceOrder
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum PlaceOrderEnumResponse {
		/// message: PlaceOrderResponse, payload: PlaceOrderResponse
	PlaceOrderResponse(PlaceOrderResponse),
	/// message: ErrorResponse, payload: ErrorResponse
	ErrorResponse(ErrorResponse),
}

/// union response for channel: PlaceMultipleOrder
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum PlaceMultipleOrderEnumResponse {
		/// message: PlaceOrderResponse, payload: PlaceOrderResponse
	PlaceOrderResponse(PlaceOrderResponse),
	/// message: ErrorResponse, payload: ErrorResponse
	ErrorResponse(ErrorResponse),
}

/// union response for channel: CancelOrder
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum CancelOrderEnumResponse {
		/// message: CancelOrderResponse, payload: CancelOrderResponse
	CancelOrderResponse(CancelOrderResponse),
	/// message: ErrorResponse, payload: ErrorResponse
	ErrorResponse(ErrorResponse),
}

/// union response for channel: CancelMultiOrder
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum CancelMultiOrderEnumResponse {
		/// message: CancelOrderResponse, payload: CancelOrderResponse
	CancelOrderResponse(CancelOrderResponse),
	/// message: ErrorResponse, payload: ErrorResponse
	ErrorResponse(ErrorResponse),
}

/// union response for channel: AmendOrder
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum AmendOrderEnumResponse {
		/// message: AmendOrderResponse, payload: AmendOrderResponse
	AmendOrderResponse(AmendOrderResponse),
	/// message: ErrorResponse, payload: ErrorResponse
	ErrorResponse(ErrorResponse),
}

/// union response for channel: MassCancelOrder
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum MassCancelOrderEnumResponse {
		/// message: MassCancelResponse, payload: MassCancelResponse
	MassCancelResponse(MassCancelResponse),
	/// message: ErrorResponse, payload: ErrorResponse
	ErrorResponse(ErrorResponse),
}

/// union response for channel: AlgoOrders
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum AlgoOrdersEnumResponse {
		/// message: AlgoOrderCancelResponse, payload: AlgoOrderResponse
	AlgoOrderCancelResponse(AlgoOrderResponse),
	/// message: AlgoOrderPushResponse, payload: AlgoOrderPushResponse
	AlgoOrderPushResponse(AlgoOrderPushResponse),
	/// message: ErrorResponse, payload: ErrorResponse
	ErrorResponse(ErrorResponse),
}

/// union response for channel: AdvanceAlgoOrders
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum AdvanceAlgoOrdersEnumResponse {
		/// message: AlgoOrderCancelResponse, payload: AlgoOrderResponse
	AlgoOrderCancelResponse(AlgoOrderResponse),
	/// message: AlgoOrderPushResponse, payload: AlgoOrderPushResponse
	AlgoOrderPushResponse(AlgoOrderPushResponse),
	/// message: ErrorResponse, payload: ErrorResponse
	ErrorResponse(ErrorResponse),
}

/// union response for channel: SpotGridOrders
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum SpotGridOrdersEnumResponse {
		/// message: SpotOrderCancelResponse, payload: AlgoOrderResponse
	SpotOrderCancelResponse(AlgoOrderResponse),
	/// message: SpotOrderPushResponse, payload: SpotGridPushResponse
	SpotOrderPushResponse(SpotGridPushResponse),
	/// message: ErrorResponse, payload: ErrorResponse
	ErrorResponse(ErrorResponse),
}

/// union response for channel: ContractGridOrders
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ContractGridOrdersEnumResponse {
		/// message: ContractOrderCancelResponse, payload: AlgoOrderResponse
	ContractOrderCancelResponse(AlgoOrderResponse),
	/// message: ContractOrderPushResponse, payload: ContractGridPushResponse
	ContractOrderPushResponse(ContractGridPushResponse),
	/// message: ErrorResponse, payload: ErrorResponse
	ErrorResponse(ErrorResponse),
}

/// union response for channel: GridPositionOrders
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum GridPositionOrdersEnumResponse {
		/// message: GridPositionOrderCancelResponse, payload: AlgoOrderResponse
	GridPositionOrderCancelResponse(AlgoOrderResponse),
	/// message: GridPositionOrderPushResponse, payload: GridPositionPushResponse
	GridPositionOrderPushResponse(GridPositionPushResponse),
	/// message: ErrorResponse, payload: ErrorResponse
	ErrorResponse(ErrorResponse),
}

/// union response for channel: GridSuborderOrders
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum GridSuborderOrdersEnumResponse {
		/// message: GridOrderCancelResponse, payload: AlgoOrderResponse
	GridOrderCancelResponse(AlgoOrderResponse),
	/// message: GridOrderPushResponse, payload: GridSuborderPushResponse
	GridOrderPushResponse(GridSuborderPushResponse),
	/// message: ErrorResponse, payload: ErrorResponse
	ErrorResponse(ErrorResponse),
}

/// union response for channel: RecurringOrders
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum RecurringOrdersEnumResponse {
		/// message: RecurringOrderCancelResponse, payload: AlgoOrderResponse
	RecurringOrderCancelResponse(AlgoOrderResponse),
	/// message: RecurringOrderPushResponse, payload: RecurringOrderResponse
	RecurringOrderPushResponse(RecurringOrderResponse),
	/// message: ErrorResponse, payload: ErrorResponse
	ErrorResponse(ErrorResponse),
}

/// union response for channel: CopyTrading
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum CopyTradingEnumResponse {
		/// message: CopyTradingResponse, payload: AlgoOrderResponse
	CopyTradingResponse(AlgoOrderResponse),
	/// message: CopyTradingPushResponse, payload: CopyTradingResponse
	CopyTradingPushResponse(CopyTradingResponse),
	/// message: ErrorResponse, payload: ErrorResponse
	ErrorResponse(ErrorResponse),
}

/// union response for channel: LeadTrading
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum LeadTradingEnumResponse {
		/// message: LeadTradingResponse, payload: AlgoOrderResponse
	LeadTradingResponse(AlgoOrderResponse),
	/// message: LeadTradingPushResponse, payload: LeadTradingResponse
	LeadTradingPushResponse(LeadTradingResponse),
	/// message: ErrorResponse, payload: ErrorResponse
	ErrorResponse(ErrorResponse),
}

/// union response for channel: Tickers
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum TickersEnumResponse {
		/// message: TickersResponse, payload: AlgoOrderResponse
	TickersResponse(AlgoOrderResponse),
	/// message: TickersPushResponse, payload: TickersResponse
	TickersPushResponse(TickersResponse),
	/// message: ErrorResponse, payload: ErrorResponse
	ErrorResponse(ErrorResponse),
}

/// union response for channel: CandleStick
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum CandleStickEnumResponse {
		/// message: CandlestickResponse, payload: AlgoOrderResponse
	CandlestickResponse(AlgoOrderResponse),
	/// message: CandlestickPushResponse, payload: CandleStickResponse
	CandlestickPushResponse(CandleStickResponse),
	/// message: ErrorResponse, payload: ErrorResponse
	ErrorResponse(ErrorResponse),
}

/// union response for channel: Trades
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum TradesEnumResponse {
		/// message: TradingResponse, payload: AlgoOrderResponse
	TradingResponse(AlgoOrderResponse),
	/// message: TradingPushResponse, payload: TradesResponse
	TradingPushResponse(TradesResponse),
	/// message: ErrorResponse, payload: ErrorResponse
	ErrorResponse(ErrorResponse),
}

/// union response for channel: AllTrades
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum AllTradesEnumResponse {
		/// message: AllTradesResponse, payload: AlgoOrderResponse
	AllTradesResponse(AlgoOrderResponse),
	/// message: AllTradesPushResponse, payload: AllTradesResponse
	AllTradesPushResponse(AllTradesResponse),
	/// message: ErrorResponse, payload: ErrorResponse
	ErrorResponse(ErrorResponse),
}

/// union response for channel: OrderBook
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum OrderBookEnumResponse {
		/// message: OrderbookResponse, payload: AlgoOrderResponse
	OrderbookResponse(AlgoOrderResponse),
	/// message: OrderbookPushResponse, payload: OrderbookResponse
	OrderbookPushResponse(OrderbookResponse),
	/// message: ErrorResponse, payload: ErrorResponse
	ErrorResponse(ErrorResponse),
}

/// union response for channel: OptionTrades
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum OptionTradesEnumResponse {
		/// message: OptionTradesResponse, payload: AlgoOrderResponse
	OptionTradesResponse(AlgoOrderResponse),
	/// message: OptionTradesPushResponse, payload: OptionTradesResponse
	OptionTradesPushResponse(OptionTradesResponse),
	/// message: ErrorResponse, payload: ErrorResponse
	ErrorResponse(ErrorResponse),
}

/// union response for channel: Auction
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum AuctionEnumResponse {
		/// message: AuctionResponse, payload: AlgoOrderResponse
	AuctionResponse(AlgoOrderResponse),
	/// message: AuctionPushResponse, payload: AuctionResponse
	AuctionPushResponse(AuctionResponse),
	/// message: ErrorResponse, payload: ErrorResponse
	ErrorResponse(ErrorResponse),
}

/// union response for channel: PlaceOrderSpread
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum PlaceOrderSpreadEnumResponse {
		/// message: SpreadOrderResponse, payload: AlgoOrderResponse
	SpreadOrderResponse(AlgoOrderResponse),
	/// message: SpreadOrderPushResponse, payload: SpreadOrderResponse
	SpreadOrderPushResponse(SpreadOrderResponse),
	/// message: ErrorResponse, payload: ErrorResponse
	ErrorResponse(ErrorResponse),
}

/// union response for channel: AmendOrderSpread
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum AmendOrderSpreadEnumResponse {
		/// message: SpreadAmendResponse, payload: SpreadAmendResponse
	SpreadAmendResponse(SpreadAmendResponse),
	/// message: ErrorResponse, payload: ErrorResponse
	ErrorResponse(ErrorResponse),
}

/// union response for channel: CancelOrderSpread
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum CancelOrderSpreadEnumResponse {
		/// message: SpreadCancelResponse, payload: SpreadAmendResponse
	SpreadCancelResponse(SpreadAmendResponse),
	/// message: ErrorResponse, payload: ErrorResponse
	ErrorResponse(ErrorResponse),
}

/// union response for channel: CancelAllOrderSpread
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum CancelAllOrderSpreadEnumResponse {
		/// message: SpreadCancelResponse, payload: SpreadCancelAllResponse
	SpreadCancelResponse(SpreadCancelAllResponse),
	/// message: ErrorResponse, payload: ErrorResponse
	ErrorResponse(ErrorResponse),
}

/// union response for channel: OrderChannel
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum OrderChannelEnumResponse {
		/// message: OrderChannelResponse, payload: OrderChannelResponse
	OrderChannelResponse(OrderChannelResponse),
	/// message: OrderChannelPushResponse, payload: OrderChannelPushResponse
	OrderChannelPushResponse(OrderChannelPushResponse),
	/// message: ErrorResponse, payload: ErrorResponse
	ErrorResponse(ErrorResponse),
}

/// union response for channel: TradeChannel
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum TradeChannelEnumResponse {
		/// message: TradeChannelResponse, payload: OrderChannelResponse
	TradeChannelResponse(OrderChannelResponse),
	/// message: TradeChannelPushResponse, payload: TradeChannelPushResponse
	TradeChannelPushResponse(TradeChannelPushResponse),
	/// message: ErrorResponse, payload: ErrorResponse
	ErrorResponse(ErrorResponse),
}

/// union response for channel: InstrumentsChannel
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum InstrumentsChannelEnumResponse {
		/// message: InstrumentResponse, payload: OrderChannelResponse
	InstrumentResponse(OrderChannelResponse),
	/// message: InstrumentPushResponse, payload: InstrumentPushResponse
	InstrumentPushResponse(InstrumentPushResponse),
	/// message: ErrorResponse, payload: ErrorResponse
	ErrorResponse(ErrorResponse),
}

/// union response for channel: OpenInterestChannel
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum OpenInterestChannelEnumResponse {
		/// message: OpenInterestResponse, payload: OrderChannelResponse
	OpenInterestResponse(OrderChannelResponse),
	/// message: OpenInterestPushResponse, payload: OpenInterestPushResponse
	OpenInterestPushResponse(OpenInterestPushResponse),
	/// message: ErrorResponse, payload: ErrorResponse
	ErrorResponse(ErrorResponse),
}

/// union response for channel: FundingRateChannel
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum FundingRateChannelEnumResponse {
		/// message: FundingResponse, payload: OrderChannelResponse
	FundingResponse(OrderChannelResponse),
	/// message: FundingratePushResponse, payload: FundingratePushResponse
	FundingratePushResponse(FundingratePushResponse),
	/// message: ErrorResponse, payload: ErrorResponse
	ErrorResponse(ErrorResponse),
}

/// union response for channel: PriceLimitChannel
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum PriceLimitChannelEnumResponse {
		/// message: PriceLimitResponse, payload: OrderChannelResponse
	PriceLimitResponse(OrderChannelResponse),
	/// message: PriceLimitPushResponse, payload: PriceLimitPushResponse
	PriceLimitPushResponse(PriceLimitPushResponse),
	/// message: ErrorResponse, payload: ErrorResponse
	ErrorResponse(ErrorResponse),
}

/// union response for channel: OptionSummaryChannel
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum OptionSummaryChannelEnumResponse {
		/// message: OptionResponse, payload: OrderChannelResponse
	OptionResponse(OrderChannelResponse),
	/// message: OptionPushResponse, payload: OptionPushResponse
	OptionPushResponse(OptionPushResponse),
	/// message: ErrorResponse, payload: ErrorResponse
	ErrorResponse(ErrorResponse),
}

/// union response for channel: PriceChannel
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum PriceChannelEnumResponse {
		/// message: PriceResponse, payload: RiskResponse
	PriceResponse(RiskResponse),
	/// message: PricePushResponse, payload: PricePushResponse
	PricePushResponse(PricePushResponse),
	/// message: ErrorResponse, payload: ErrorResponse
	ErrorResponse(ErrorResponse),
}

/// union response for channel: MarkPriceChannel
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum MarkPriceChannelEnumResponse {
		/// message: MarkPriceResponse, payload: AlgoOrderResponse
	MarkPriceResponse(AlgoOrderResponse),
	/// message: MarkPricePushResponse, payload: MarkPricePushResponse
	MarkPricePushResponse(MarkPricePushResponse),
	/// message: ErrorResponse, payload: ErrorResponse
	ErrorResponse(ErrorResponse),
}

/// union response for channel: IndexTickerChannel
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum IndexTickerChannelEnumResponse {
		/// message: IndexTickerResponse, payload: AlgoOrderResponse
	IndexTickerResponse(AlgoOrderResponse),
	/// message: IndexTickerPushResponse, payload: PublicTickersPushResponse
	IndexTickerPushResponse(PublicTickersPushResponse),
	/// message: ErrorResponse, payload: ErrorResponse
	ErrorResponse(ErrorResponse),
}

/// union response for channel: MarkCandlestick
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum MarkCandlestickEnumResponse {
		/// message: MarkCandlestickResponse, payload: AlgoOrderResponse
	MarkCandlestickResponse(AlgoOrderResponse),
	/// message: MarkCandlestickPushResponse, payload: PublicCandlestickPushResponse
	MarkCandlestickPushResponse(PublicCandlestickPushResponse),
	/// message: ErrorResponse, payload: ErrorResponse
	ErrorResponse(ErrorResponse),
}

/// union response for channel: LiquidationOrders
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum LiquidationOrdersEnumResponse {
		/// message: LiquidationResponse, payload: LiquidationResponse
	LiquidationResponse(LiquidationResponse),
	/// message: ErrorResponse, payload: ErrorResponse
	ErrorResponse(ErrorResponse),
}

/// union response for channel: Adl
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum AdlEnumResponse {
		/// message: AdlResponse, payload: AdlResponse
	AdlResponse(AdlResponse),
	/// message: ErrorResponse, payload: ErrorResponse
	ErrorResponse(ErrorResponse),
}

/// union response for channel: Calender
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum CalenderEnumResponse {
		/// message: CalenderResponse, payload: AlgoOrderResponse
	CalenderResponse(AlgoOrderResponse),
	/// message: CalenderPushResponse, payload: CalendarPushResponse
	CalenderPushResponse(CalendarPushResponse),
	/// message: ErrorResponse, payload: ErrorResponse
	ErrorResponse(ErrorResponse),
}

/// union response for channel: Deposit
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum DepositEnumResponse {
		/// message: DepositResponse, payload: DepositResponseData
	DepositResponse(DepositResponseData),
	/// message: DepositPushResponse, payload: DepositPushResponse
	DepositPushResponse(DepositPushResponse),
	/// message: ErrorResponse, payload: ErrorResponse
	ErrorResponse(ErrorResponse),
}

/// union response for channel: Withdrawal
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum WithdrawalEnumResponse {
		/// message: WithdrawResponse, payload: DepositResponseData
	WithdrawResponse(DepositResponseData),
	/// message: WithdrawPushResponse, payload: WithdrawPushResponse
	WithdrawPushResponse(WithdrawPushResponse),
	/// message: ErrorResponse, payload: ErrorResponse
	ErrorResponse(ErrorResponse),
}

#[derive(Debug)]
pub struct OkxWebsocketBusinessEndpointsClient {
	base_url: String,
}

impl OkxWebsocketBusinessEndpointsClient {
	/// connect to the okx websocket server
	pub async fn new() -> Self {
		Self {
			base_url: "wss://ws.okx.com:8443/ws/v5/business".to_string(),
 		}
	}
/// Private WebSocket channel for operations.  
/// send: SubscribeRequest  
/// receive: AccountEnumResponse
pub async fn account(& mut self) -> Result<Stream<SubscribeRequest, AccountEnumResponse>> {
		let endpooint_url = format!("{}{}", self.base_url, "account");

		let(ws_stream, _) = connect_async(endpooint_url).await.map_err(| err | {
			eprintln!("Failed to connect: {:?}", err);
		err
		}) ?;

		Ok(TypedWebSocketStream::new(ws_stream))
}

/// Private WebSocket channel for operations.  
/// send: PositionRequest  
/// receive: PositionsEnumResponse
pub async fn positions(& mut self) -> Result<Stream<PositionRequest, PositionsEnumResponse>> {
		let endpooint_url = format!("{}{}", self.base_url, "undefined");

		let(ws_stream, _) = connect_async(endpooint_url).await.map_err(| err | {
			eprintln!("Failed to connect: {:?}", err);
		err
		}) ?;

		Ok(TypedWebSocketStream::new(ws_stream))
}

/// Private WebSocket channel for operations.  
/// send: BalanceRequest  
/// receive: BalancesEnumResponse
pub async fn balances(& mut self) -> Result<Stream<BalanceRequest, BalancesEnumResponse>> {
		let endpooint_url = format!("{}{}", self.base_url, "undefined");

		let(ws_stream, _) = connect_async(endpooint_url).await.map_err(| err | {
			eprintln!("Failed to connect: {:?}", err);
		err
		}) ?;

		Ok(TypedWebSocketStream::new(ws_stream))
}

/// Private WebSocket channel for operations.  
/// send: RiskRequest  
/// receive: PositionRiskEnumResponse
pub async fn position_risk(& mut self) -> Result<Stream<RiskRequest, PositionRiskEnumResponse>> {
		let endpooint_url = format!("{}{}", self.base_url, "undefined");

		let(ws_stream, _) = connect_async(endpooint_url).await.map_err(| err | {
			eprintln!("Failed to connect: {:?}", err);
		err
		}) ?;

		Ok(TypedWebSocketStream::new(ws_stream))
}

/// Private WebSocket channel for operations.  
/// send: OrderRequest  
/// receive: OrderEnumResponse
pub async fn order(& mut self) -> Result<Stream<OrderRequest, OrderEnumResponse>> {
		let endpooint_url = format!("{}{}", self.base_url, "undefined");

		let(ws_stream, _) = connect_async(endpooint_url).await.map_err(| err | {
			eprintln!("Failed to connect: {:?}", err);
		err
		}) ?;

		Ok(TypedWebSocketStream::new(ws_stream))
}

/// Private WebSocket channel for operations.  
/// send: FillsRequest  
/// receive: FillsEnumResponse
pub async fn fills(& mut self) -> Result<Stream<FillsRequest, FillsEnumResponse>> {
		let endpooint_url = format!("{}{}", self.base_url, "undefined");

		let(ws_stream, _) = connect_async(endpooint_url).await.map_err(| err | {
			eprintln!("Failed to connect: {:?}", err);
		err
		}) ?;

		Ok(TypedWebSocketStream::new(ws_stream))
}

/// Private WebSocket channel for operations.  
/// send: PlaceOrderRequest  
/// receive: PlaceOrderEnumResponse
pub async fn place_order(& mut self) -> Result<Stream<PlaceOrderRequest, PlaceOrderEnumResponse>> {
		let endpooint_url = format!("{}{}", self.base_url, "undefined");

		let(ws_stream, _) = connect_async(endpooint_url).await.map_err(| err | {
			eprintln!("Failed to connect: {:?}", err);
		err
		}) ?;

		Ok(TypedWebSocketStream::new(ws_stream))
}

/// Place multiple orders  
/// send: PlaceOrderRequest  
/// receive: PlaceMultipleOrderEnumResponse
pub async fn place_multiple_order(& mut self) -> Result<Stream<PlaceOrderRequest, PlaceMultipleOrderEnumResponse>> {
		let endpooint_url = format!("{}{}", self.base_url, "undefined");

		let(ws_stream, _) = connect_async(endpooint_url).await.map_err(| err | {
			eprintln!("Failed to connect: {:?}", err);
		err
		}) ?;

		Ok(TypedWebSocketStream::new(ws_stream))
}

/// Cancel orders  
/// send: CancelOrderRequest  
/// receive: CancelOrderEnumResponse
pub async fn cancel_order(& mut self) -> Result<Stream<CancelOrderRequest, CancelOrderEnumResponse>> {
		let endpooint_url = format!("{}{}", self.base_url, "undefined");

		let(ws_stream, _) = connect_async(endpooint_url).await.map_err(| err | {
			eprintln!("Failed to connect: {:?}", err);
		err
		}) ?;

		Ok(TypedWebSocketStream::new(ws_stream))
}

/// Cancel multiple orders  
/// send: CancelOrderRequest  
/// receive: CancelMultiOrderEnumResponse
pub async fn cancel_multi_order(& mut self) -> Result<Stream<CancelOrderRequest, CancelMultiOrderEnumResponse>> {
		let endpooint_url = format!("{}{}", self.base_url, "undefined");

		let(ws_stream, _) = connect_async(endpooint_url).await.map_err(| err | {
			eprintln!("Failed to connect: {:?}", err);
		err
		}) ?;

		Ok(TypedWebSocketStream::new(ws_stream))
}

/// Amend orders  
/// send: AmendOrderRequest  
/// receive: AmendOrderEnumResponse
pub async fn amend_order(& mut self) -> Result<Stream<AmendOrderRequest, AmendOrderEnumResponse>> {
		let endpooint_url = format!("{}{}", self.base_url, "undefined");

		let(ws_stream, _) = connect_async(endpooint_url).await.map_err(| err | {
			eprintln!("Failed to connect: {:?}", err);
		err
		}) ?;

		Ok(TypedWebSocketStream::new(ws_stream))
}

/// Cancel all the MMP pending orders of an instrument family. Only applicable to Option in Portfolio Margin mode, and MMP privilege is required.  
/// send: MassCancelRequest  
/// receive: MassCancelOrderEnumResponse
pub async fn mass_cancel_order(& mut self) -> Result<Stream<MassCancelRequest, MassCancelOrderEnumResponse>> {
		let endpooint_url = format!("{}{}", self.base_url, "undefined");

		let(ws_stream, _) = connect_async(endpooint_url).await.map_err(| err | {
			eprintln!("Failed to connect: {:?}", err);
		err
		}) ?;

		Ok(TypedWebSocketStream::new(ws_stream))
}

/// Retrieve algo orders (includes trigger order, oco order, conditional order). Data will not be pushed when first subscribed. Data will only be pushed when there are order updates.  
/// send: AlgoOrdersRequest  
/// receive: AlgoOrdersEnumResponse
pub async fn algo_orders(& mut self) -> Result<Stream<AlgoOrdersRequest, AlgoOrdersEnumResponse>> {
		let endpooint_url = format!("{}{}", self.base_url, "undefined");

		let(ws_stream, _) = connect_async(endpooint_url).await.map_err(| err | {
			eprintln!("Failed to connect: {:?}", err);
		err
		}) ?;

		Ok(TypedWebSocketStream::new(ws_stream))
}

/// Retrieve advance algo orders (including Iceberg order, TWAP order, Trailing order). Data will be pushed when first subscribed. Data will be pushed when triggered by events such as placing/canceling order.  
/// send: AdvanceAlgoOrdersRequest  
/// receive: AdvanceAlgoOrdersEnumResponse
pub async fn advance_algo_orders(& mut self) -> Result<Stream<AdvanceAlgoOrdersRequest, AdvanceAlgoOrdersEnumResponse>> {
		let endpooint_url = format!("{}{}", self.base_url, "undefined");

		let(ws_stream, _) = connect_async(endpooint_url).await.map_err(| err | {
			eprintln!("Failed to connect: {:?}", err);
		err
		}) ?;

		Ok(TypedWebSocketStream::new(ws_stream))
}

/// Retrieve spot grid algo orders. Data will be pushed when triggered by events such as placing/canceling order. It will also be pushed in regular interval according to subscription granularity.  
/// send: AdvanceAlgoOrdersRequest  
/// receive: SpotGridOrdersEnumResponse
pub async fn spot_grid_orders(& mut self) -> Result<Stream<AdvanceAlgoOrdersRequest, SpotGridOrdersEnumResponse>> {
		let endpooint_url = format!("{}{}", self.base_url, "undefined");

		let(ws_stream, _) = connect_async(endpooint_url).await.map_err(| err | {
			eprintln!("Failed to connect: {:?}", err);
		err
		}) ?;

		Ok(TypedWebSocketStream::new(ws_stream))
}

/// Retrieve contract grid algo orders. Data will be pushed when triggered by events such as placing/canceling order. It will also be pushed in regular interval according to subscription granularity.  
/// send: AdvanceAlgoOrdersRequest  
/// receive: ContractGridOrdersEnumResponse
pub async fn contract_grid_orders(& mut self) -> Result<Stream<AdvanceAlgoOrdersRequest, ContractGridOrdersEnumResponse>> {
		let endpooint_url = format!("{}{}", self.base_url, "undefined");

		let(ws_stream, _) = connect_async(endpooint_url).await.map_err(| err | {
			eprintln!("Failed to connect: {:?}", err);
		err
		}) ?;

		Ok(TypedWebSocketStream::new(ws_stream))
}

/// Retrieve contract grid positions. Data will be pushed when triggered by events such as placing/canceling order. Please ignore the empty data.  
/// send: AdvanceAlgoOrdersRequest  
/// receive: GridPositionOrdersEnumResponse
pub async fn grid_position_orders(& mut self) -> Result<Stream<AdvanceAlgoOrdersRequest, GridPositionOrdersEnumResponse>> {
		let endpooint_url = format!("{}{}", self.base_url, "undefined");

		let(ws_stream, _) = connect_async(endpooint_url).await.map_err(| err | {
			eprintln!("Failed to connect: {:?}", err);
		err
		}) ?;

		Ok(TypedWebSocketStream::new(ws_stream))
}

/// Retrieve grid sub orders. Data will be pushed when triggered by events such as placing order. Please ignore the empty data.  
/// send: AdvanceAlgoOrdersRequest  
/// receive: GridSuborderOrdersEnumResponse
pub async fn grid_suborder_orders(& mut self) -> Result<Stream<AdvanceAlgoOrdersRequest, GridSuborderOrdersEnumResponse>> {
		let endpooint_url = format!("{}{}", self.base_url, "undefined");

		let(ws_stream, _) = connect_async(endpooint_url).await.map_err(| err | {
			eprintln!("Failed to connect: {:?}", err);
		err
		}) ?;

		Ok(TypedWebSocketStream::new(ws_stream))
}

/// Retrieve recurring buy orders. Data will be pushed when triggered by events. It will also be pushed in regular interval according to subscription granularity.  
/// send: AdvanceAlgoOrdersRequest  
/// receive: RecurringOrdersEnumResponse
pub async fn recurring_orders(& mut self) -> Result<Stream<AdvanceAlgoOrdersRequest, RecurringOrdersEnumResponse>> {
		let endpooint_url = format!("{}{}", self.base_url, "undefined");

		let(ws_stream, _) = connect_async(endpooint_url).await.map_err(| err | {
			eprintln!("Failed to connect: {:?}", err);
		err
		}) ?;

		Ok(TypedWebSocketStream::new(ws_stream))
}

/// As a copy trader, receive push notification of copy trading.  
/// send: AlgoOrdersRequest  
/// receive: CopyTradingEnumResponse
pub async fn copy_trading(& mut self) -> Result<Stream<AlgoOrdersRequest, CopyTradingEnumResponse>> {
		let endpooint_url = format!("{}{}", self.base_url, "undefined");

		let(ws_stream, _) = connect_async(endpooint_url).await.map_err(| err | {
			eprintln!("Failed to connect: {:?}", err);
		err
		}) ?;

		Ok(TypedWebSocketStream::new(ws_stream))
}

/// The notification when failing to lead trade.  
/// send: AlgoOrdersRequest  
/// receive: LeadTradingEnumResponse
pub async fn lead_trading(& mut self) -> Result<Stream<AlgoOrdersRequest, LeadTradingEnumResponse>> {
		let endpooint_url = format!("{}{}", self.base_url, "undefined");

		let(ws_stream, _) = connect_async(endpooint_url).await.map_err(| err | {
			eprintln!("Failed to connect: {:?}", err);
		err
		}) ?;

		Ok(TypedWebSocketStream::new(ws_stream))
}

/// Retrieve the last traded price, bid price, ask price and 24-hour trading volume of instruments.The fastest rate is 1 update/100ms. There will be no update if the event is not triggered. The events which can trigger update: trade, the change on best ask/bid.  
/// send: TickersRequest  
/// receive: TickersEnumResponse
pub async fn tickers(& mut self) -> Result<Stream<TickersRequest, TickersEnumResponse>> {
		let endpooint_url = format!("{}{}", self.base_url, "undefined");

		let(ws_stream, _) = connect_async(endpooint_url).await.map_err(| err | {
			eprintln!("Failed to connect: {:?}", err);
		err
		}) ?;

		Ok(TypedWebSocketStream::new(ws_stream))
}

/// Retrieve the candlesticks data of an instrument. the push frequency is the fastest interval 1 second push the data.  
/// send: TickersRequest  
/// receive: CandleStickEnumResponse
pub async fn candle_stick(& mut self) -> Result<Stream<TickersRequest, CandleStickEnumResponse>> {
		let endpooint_url = format!("{}{}", self.base_url, "undefined");

		let(ws_stream, _) = connect_async(endpooint_url).await.map_err(| err | {
			eprintln!("Failed to connect: {:?}", err);
		err
		}) ?;

		Ok(TypedWebSocketStream::new(ws_stream))
}

/// Retrieve the recent trades data. Data will be pushed whenever there is a trade. Every update may aggregate multiple trades.The message is sent only once per taker order, per filled price. The count field is used to represent the number of aggregated matches.  
/// send: TickersRequest  
/// receive: TradesEnumResponse
pub async fn trades(& mut self) -> Result<Stream<TickersRequest, TradesEnumResponse>> {
		let endpooint_url = format!("{}{}", self.base_url, "undefined");

		let(ws_stream, _) = connect_async(endpooint_url).await.map_err(| err | {
			eprintln!("Failed to connect: {:?}", err);
		err
		}) ?;

		Ok(TypedWebSocketStream::new(ws_stream))
}

/// Retrieve the recent trades data. Data will be pushed whenever there is a trade. Every update contain only one trade.  
/// send: TickersRequest  
/// receive: AllTradesEnumResponse
pub async fn all_trades(& mut self) -> Result<Stream<TickersRequest, AllTradesEnumResponse>> {
		let endpooint_url = format!("{}{}", self.base_url, "undefined");

		let(ws_stream, _) = connect_async(endpooint_url).await.map_err(| err | {
			eprintln!("Failed to connect: {:?}", err);
		err
		}) ?;

		Ok(TypedWebSocketStream::new(ws_stream))
}

/// Retrieve order book data.<br>Use books for 400 depth levels, books5 for 5 depth levels, bbo-tbt tick-by-tick 1 depth level, books50-l2-tbt tick-by-tick 50 depth levels, and books-l2-tbt for tick-by-tick 400 depth levels.<br>books: 400 depth levels will be pushed in the initial full snapshot. Incremental data will be pushed every 100 ms for the changes in the order book during that period of time.<br>books5: 5 depth levels snapshot will be pushed in the initial push. Snapshot data will be pushed every 100 ms when there are changes in the 5 depth levels snapshot.<br>bbo-tbt: 1 depth level snapshot will be pushed in the initial push. Snapshot data will be pushed every 10 ms when there are changes in the 1 depth level snapshot.<br>books-l2-tbt: 400 depth levels will be pushed in the initial full snapshot. Incremental data will be pushed every 10 ms for the changes in the order book during that period of time.<br>books50-l2-tbt: 50 depth levels will be pushed in the initial full snapshot. Incremental data will be pushed every 10 ms for the changes in the order book during that period of time.<br>The push sequence for order book channels within the same connection and trading symbols is fixed as:<br>bbo-tbt -> books-l2-tbt -> books50-l2-tbt -> books -> books5.<br>Users can not simultaneously subscribe to books-l2-tbt and books50-l2-tbt/books  
/// send: TickersRequest  
/// receive: OrderBookEnumResponse
pub async fn order_book(& mut self) -> Result<Stream<TickersRequest, OrderBookEnumResponse>> {
		let endpooint_url = format!("{}{}", self.base_url, "undefined");

		let(ws_stream, _) = connect_async(endpooint_url).await.map_err(| err | {
			eprintln!("Failed to connect: {:?}", err);
		err
		}) ?;

		Ok(TypedWebSocketStream::new(ws_stream))
}

/// Retrieve the recent trades data. Data will be pushed whenever there is a trade. Every update contain only one trade.  
/// send: AlgoOrdersRequest  
/// receive: OptionTradesEnumResponse
pub async fn option_trades(& mut self) -> Result<Stream<AlgoOrdersRequest, OptionTradesEnumResponse>> {
		let endpooint_url = format!("{}{}", self.base_url, "undefined");

		let(ws_stream, _) = connect_async(endpooint_url).await.map_err(| err | {
			eprintln!("Failed to connect: {:?}", err);
		err
		}) ?;

		Ok(TypedWebSocketStream::new(ws_stream))
}

/// Retrieve call auction details.  
/// send: AlgoOrdersRequest  
/// receive: AuctionEnumResponse
pub async fn auction(& mut self) -> Result<Stream<AlgoOrdersRequest, AuctionEnumResponse>> {
		let endpooint_url = format!("{}{}", self.base_url, "undefined");

		let(ws_stream, _) = connect_async(endpooint_url).await.map_err(| err | {
			eprintln!("Failed to connect: {:?}", err);
		err
		}) ?;

		Ok(TypedWebSocketStream::new(ws_stream))
}

/// You can place an order only if you have sufficient funds.  
/// send: SpreadOrderRequest  
/// receive: PlaceOrderSpreadEnumResponse
pub async fn place_order_spread(& mut self) -> Result<Stream<SpreadOrderRequest, PlaceOrderSpreadEnumResponse>> {
		let endpooint_url = format!("{}{}", self.base_url, "undefined");

		let(ws_stream, _) = connect_async(endpooint_url).await.map_err(| err | {
			eprintln!("Failed to connect: {:?}", err);
		err
		}) ?;

		Ok(TypedWebSocketStream::new(ws_stream))
}

/// Amend an incomplete order.  
/// send: SpreadAmendOrderRequest  
/// receive: AmendOrderSpreadEnumResponse
pub async fn amend_order_spread(& mut self) -> Result<Stream<SpreadAmendOrderRequest, AmendOrderSpreadEnumResponse>> {
		let endpooint_url = format!("{}{}", self.base_url, "undefined");

		let(ws_stream, _) = connect_async(endpooint_url).await.map_err(| err | {
			eprintln!("Failed to connect: {:?}", err);
		err
		}) ?;

		Ok(TypedWebSocketStream::new(ws_stream))
}

/// cancel an order  
/// send: SpreadCancelOrderRequest  
/// receive: CancelOrderSpreadEnumResponse
pub async fn cancel_order_spread(& mut self) -> Result<Stream<SpreadCancelOrderRequest, CancelOrderSpreadEnumResponse>> {
		let endpooint_url = format!("{}{}", self.base_url, "undefined");

		let(ws_stream, _) = connect_async(endpooint_url).await.map_err(| err | {
			eprintln!("Failed to connect: {:?}", err);
		err
		}) ?;

		Ok(TypedWebSocketStream::new(ws_stream))
}

/// cancel all orders  
/// send: SpreadCancelAllRequest  
/// receive: CancelAllOrderSpreadEnumResponse
pub async fn cancel_all_order_spread(& mut self) -> Result<Stream<SpreadCancelAllRequest, CancelAllOrderSpreadEnumResponse>> {
		let endpooint_url = format!("{}{}", self.base_url, "undefined");

		let(ws_stream, _) = connect_async(endpooint_url).await.map_err(| err | {
			eprintln!("Failed to connect: {:?}", err);
		err
		}) ?;

		Ok(TypedWebSocketStream::new(ws_stream))
}

/// Private Order Channel  
/// send: OrderChannelRequest  
/// receive: OrderChannelEnumResponse
pub async fn order_channel(& mut self) -> Result<Stream<OrderChannelRequest, OrderChannelEnumResponse>> {
		let endpooint_url = format!("{}{}", self.base_url, "undefined");

		let(ws_stream, _) = connect_async(endpooint_url).await.map_err(| err | {
			eprintln!("Failed to connect: {:?}", err);
		err
		}) ?;

		Ok(TypedWebSocketStream::new(ws_stream))
}

/// Private Trade Channel  
/// send: OrderChannelRequest  
/// receive: TradeChannelEnumResponse
pub async fn trade_channel(& mut self) -> Result<Stream<OrderChannelRequest, TradeChannelEnumResponse>> {
		let endpooint_url = format!("{}{}", self.base_url, "undefined");

		let(ws_stream, _) = connect_async(endpooint_url).await.map_err(| err | {
			eprintln!("Failed to connect: {:?}", err);
		err
		}) ?;

		Ok(TypedWebSocketStream::new(ws_stream))
}

/// Retrieve order book data. Available channels:<br>sprd-bbo-tbt: 1 depth level snapshot will be pushed in the initial push. Snapshot data will be pushed every 10 ms when there are changes in the 1 depth level snapshot.<br>sprd-books5: 5 depth levels snapshot will be pushed in the initial push. Snapshot data will be pushed every 100 ms when there are changes in the 5 depth levels snapshot.<br>sprd-books-l2-tbt: 400 depth levels will be pushed in the initial full snapshot. Incremental data will be pushed every 10 ms for the changes in the order book during that period of time.<br>The push sequence for order book channels within the same connection and trading symbols is fixed as: sprd-bbo-tbt -> sprd-books-l2-tbt -> sprd-books5.
pub async fn order_book_channel(&self) {
	todo!("unmapped case, check the logic and implement this case, send=0, receive=0");
}

/// Retrieve the recent trades data from sprd-public-trades. Data will be pushed whenever there is a trade. Every update contains only one trade.
pub async fn public_trade_channel(&self) {
	todo!("unmapped case, check the logic and implement this case, send=0, receive=0");
}

/// Retrieve the last traded price, bid price, ask price. The fastest rate is 1 update/100ms. There will be no update if the event is not triggered. The events which can trigger update: trade, the change on best ask/bid price
pub async fn public_tickers_channel(&self) {
	todo!("unmapped case, check the logic and implement this case, send=0, receive=0");
}

/// Retrieve the candlesticks data of an instrument. The push frequency is the fastest interval 1 second push the data.
pub async fn public_candlestick_channel(&self) {
	todo!("unmapped case, check the logic and implement this case, send=0, receive=0");
}

/// The instruments will be pushed if there is any change to the instrument’s state (such as delivery of FUTURES, exercise of OPTION, listing of new contracts / trading pairs, trading suspension, etc.)  
/// send: InstrumentRequest  
/// receive: InstrumentsChannelEnumResponse
pub async fn instruments_channel(& mut self) -> Result<Stream<InstrumentRequest, InstrumentsChannelEnumResponse>> {
		let endpooint_url = format!("{}{}", self.base_url, "undefined");

		let(ws_stream, _) = connect_async(endpooint_url).await.map_err(| err | {
			eprintln!("Failed to connect: {:?}", err);
		err
		}) ?;

		Ok(TypedWebSocketStream::new(ws_stream))
}

/// Retrieve the open interest. Data will be pushed every 3 seconds when there are updates.  
/// send: OrderChannelRequest  
/// receive: OpenInterestChannelEnumResponse
pub async fn open_interest_channel(& mut self) -> Result<Stream<OrderChannelRequest, OpenInterestChannelEnumResponse>> {
		let endpooint_url = format!("{}{}", self.base_url, "undefined");

		let(ws_stream, _) = connect_async(endpooint_url).await.map_err(| err | {
			eprintln!("Failed to connect: {:?}", err);
		err
		}) ?;

		Ok(TypedWebSocketStream::new(ws_stream))
}

/// Retrieve funding rate. Data will be pushed in 30s to 90s.  
/// send: OrderChannelRequest  
/// receive: FundingRateChannelEnumResponse
pub async fn funding_rate_channel(& mut self) -> Result<Stream<OrderChannelRequest, FundingRateChannelEnumResponse>> {
		let endpooint_url = format!("{}{}", self.base_url, "undefined");

		let(ws_stream, _) = connect_async(endpooint_url).await.map_err(| err | {
			eprintln!("Failed to connect: {:?}", err);
		err
		}) ?;

		Ok(TypedWebSocketStream::new(ws_stream))
}

/// Retrieve the maximum buy price and minimum sell price of instruments. Data will be pushed every 200ms when there are changes in limits, and will not be pushed when there is no changes on limit.  
/// send: OrderChannelRequest  
/// receive: PriceLimitChannelEnumResponse
pub async fn price_limit_channel(& mut self) -> Result<Stream<OrderChannelRequest, PriceLimitChannelEnumResponse>> {
		let endpooint_url = format!("{}{}", self.base_url, "undefined");

		let(ws_stream, _) = connect_async(endpooint_url).await.map_err(| err | {
			eprintln!("Failed to connect: {:?}", err);
		err
		}) ?;

		Ok(TypedWebSocketStream::new(ws_stream))
}

/// Retrieve detailed pricing information of all OPTION contracts. Data will be pushed at once.  
/// send: OrderChannelRequest  
/// receive: OptionSummaryChannelEnumResponse
pub async fn option_summary_channel(& mut self) -> Result<Stream<OrderChannelRequest, OptionSummaryChannelEnumResponse>> {
		let endpooint_url = format!("{}{}", self.base_url, "undefined");

		let(ws_stream, _) = connect_async(endpooint_url).await.map_err(| err | {
			eprintln!("Failed to connect: {:?}", err);
		err
		}) ?;

		Ok(TypedWebSocketStream::new(ws_stream))
}

/// Retrieve the estimated delivery/exercise price of SWAP, FUTURES and OPTION contracts.<br>Only the estimated delivery/exercise price will be pushed an hour before delivery/exercise, and will be pushed if there is any price change.  
/// send: RiskRequest  
/// receive: PriceChannelEnumResponse
pub async fn price_channel(& mut self) -> Result<Stream<RiskRequest, PriceChannelEnumResponse>> {
		let endpooint_url = format!("{}{}", self.base_url, "undefined");

		let(ws_stream, _) = connect_async(endpooint_url).await.map_err(| err | {
			eprintln!("Failed to connect: {:?}", err);
		err
		}) ?;

		Ok(TypedWebSocketStream::new(ws_stream))
}

/// Retrieve the mark price. Data will be pushed every 200 ms when the mark price changes, and will be pushed every 10 seconds when the mark price does not change.  
/// send: AlgoOrdersRequest  
/// receive: MarkPriceChannelEnumResponse
pub async fn mark_price_channel(& mut self) -> Result<Stream<AlgoOrdersRequest, MarkPriceChannelEnumResponse>> {
		let endpooint_url = format!("{}{}", self.base_url, "undefined");

		let(ws_stream, _) = connect_async(endpooint_url).await.map_err(| err | {
			eprintln!("Failed to connect: {:?}", err);
		err
		}) ?;

		Ok(TypedWebSocketStream::new(ws_stream))
}

/// Retrieve index tickers data. Push data every 100ms if there are any changes, otherwise push once a minute.  
/// send: AlgoOrdersRequest  
/// receive: IndexTickerChannelEnumResponse
pub async fn index_ticker_channel(& mut self) -> Result<Stream<AlgoOrdersRequest, IndexTickerChannelEnumResponse>> {
		let endpooint_url = format!("{}{}", self.base_url, "undefined");

		let(ws_stream, _) = connect_async(endpooint_url).await.map_err(| err | {
			eprintln!("Failed to connect: {:?}", err);
		err
		}) ?;

		Ok(TypedWebSocketStream::new(ws_stream))
}

/// Retrieve the candlesticks data of the mark price. The push frequency is the fastest interval 1 second push the data.  
/// send: AlgoOrdersRequest  
/// receive: MarkCandlestickEnumResponse
pub async fn mark_candlestick(& mut self) -> Result<Stream<AlgoOrdersRequest, MarkCandlestickEnumResponse>> {
		let endpooint_url = format!("{}{}", self.base_url, "undefined");

		let(ws_stream, _) = connect_async(endpooint_url).await.map_err(| err | {
			eprintln!("Failed to connect: {:?}", err);
		err
		}) ?;

		Ok(TypedWebSocketStream::new(ws_stream))
}

/// Retrieve the recent liquidation orders. For futures and swaps, each contract will only show a maximum of one order per one-second period. This data doesn’t represent the total number of liquidations on OKX.  
/// send: AlgoOrdersRequest  
/// receive: LiquidationOrdersEnumResponse
pub async fn liquidation_orders(& mut self) -> Result<Stream<AlgoOrdersRequest, LiquidationOrdersEnumResponse>> {
		let endpooint_url = format!("{}{}", self.base_url, "undefined");

		let(ws_stream, _) = connect_async(endpooint_url).await.map_err(| err | {
			eprintln!("Failed to connect: {:?}", err);
		err
		}) ?;

		Ok(TypedWebSocketStream::new(ws_stream))
}

/// Auto-deleveraging warning channel.<br>In the normal state, data will be pushed once every minute to display the balance of insurance fund and etc.<br>In the warning state or when there is ADL risk (warning/adl), data will be pushed every second to display information such as the real-time decline rate of insurance fund.  
/// send: AlgoOrdersRequest  
/// receive: AdlEnumResponse
pub async fn adl(& mut self) -> Result<Stream<AlgoOrdersRequest, AdlEnumResponse>> {
		let endpooint_url = format!("{}{}", self.base_url, "undefined");

		let(ws_stream, _) = connect_async(endpooint_url).await.map_err(| err | {
			eprintln!("Failed to connect: {:?}", err);
		err
		}) ?;

		Ok(TypedWebSocketStream::new(ws_stream))
}

/// Retrieve the most up-to-date economic calendar data. This endpoint is only applicable to VIP 1 and above users in the trading fee tier.  
/// send: CalendarRequest  
/// receive: CalenderEnumResponse
pub async fn calender(& mut self) -> Result<Stream<CalendarRequest, CalenderEnumResponse>> {
		let endpooint_url = format!("{}{}", self.base_url, "undefined");

		let(ws_stream, _) = connect_async(endpooint_url).await.map_err(| err | {
			eprintln!("Failed to connect: {:?}", err);
		err
		}) ?;

		Ok(TypedWebSocketStream::new(ws_stream))
}

/// A push notification is triggered when a deposit is initiated or the deposit status changes.<br>Supports subscriptions for accounts.<br>If it is a master account subscription, you can receive the push of the deposit info of both the master account and the sub-account.<br>If it is a sub-account subscription, only the push of sub-account deposit info you can receive.  
/// send: DepositRequest  
/// receive: DepositEnumResponse
pub async fn deposit(& mut self) -> Result<Stream<DepositRequest, DepositEnumResponse>> {
		let endpooint_url = format!("{}{}", self.base_url, "undefined");

		let(ws_stream, _) = connect_async(endpooint_url).await.map_err(| err | {
			eprintln!("Failed to connect: {:?}", err);
		err
		}) ?;

		Ok(TypedWebSocketStream::new(ws_stream))
}

/// A push notification is triggered when a withdrawal is initiated or the withdrawal status changes.<br>Supports subscriptions for accounts<br>If it is a master account subscription, you can receive the push of the withdrawal info of both the master account and the sub-account.<br>If it is a sub-account subscription, only the push of sub-account withdrawal info you can receive.  
/// send: DepositRequest  
/// receive: WithdrawalEnumResponse
pub async fn withdrawal(& mut self) -> Result<Stream<DepositRequest, WithdrawalEnumResponse>> {
		let endpooint_url = format!("{}{}", self.base_url, "undefined");

		let(ws_stream, _) = connect_async(endpooint_url).await.map_err(| err | {
			eprintln!("Failed to connect: {:?}", err);
		err
		}) ?;

		Ok(TypedWebSocketStream::new(ws_stream))
}
}
