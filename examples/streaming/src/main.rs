use std::{env, pin::Pin, task::{Context, Poll}};

use futures::{StreamExt, TryStreamExt, Stream};
use futures_util::stream;
use tinkoff_invest_api::{
    tcs::{
        market_data_request::Payload, CandleInstrument, InstrumentIdType, InstrumentRequest,
        InstrumentStatus, InstrumentsRequest, OrderBookInstrument, Share, ShareResponse,
        SubscribeCandlesRequest, SubscribeOrderBookRequest, SubscriptionAction,
        SubscriptionInterval,
    },
    TIResult, TinkoffInvestService,
};
use tonic;

#[tokio::main]
async fn main() -> TIResult<()> {
    let service = TinkoffInvestService::new(env::var("TINKOFF_SDK_TESTING_ENV").unwrap());
    let channel = service.create_channel().await?;
    let mut marketdata_stream = service.marketdata_stream(channel).await?;

    let (tx, rx) = flume::unbounded();
    let request = tinkoff_invest_api::tcs::MarketDataRequest {
        payload: Some(Payload::SubscribeCandlesRequest(SubscribeCandlesRequest {
            subscription_action: SubscriptionAction::Subscribe as i32,
            instruments: vec![
                CandleInstrument {figi:"BBG00YFSF9D7".to_string(), interval: SubscriptionInterval::OneMinute as i32 },
            ],
        }))
    };
    tx.send(request).unwrap();

    let response = marketdata_stream
        .market_data_stream(rx.into_stream())
        .await?;

    let mut streaming = response.into_inner();

    loop {
        if let Some(next_message) = streaming.message().await? {
            println!("MarketData {:?}", next_message);
        }
    }

    Ok(())
}
