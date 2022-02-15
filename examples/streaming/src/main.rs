use std::env;

use futures::StreamExt;
use tinkoff_invest_api::{
    TinkoffInvestService, TIResult,
    tcs::{
        SubscribeCandlesRequest, CandleInstrument, SubscriptionAction, SubscriptionInterval,
        market_data_request::{Payload}
    }
};
use futures_util::stream;

#[tokio::main]
async fn main() -> TIResult<()> {
    println!("Welcome");

    let service = TinkoffInvestService::new(env::var("TINKOFF_SDK_TESTING_ENV").unwrap());
    let channel = service.create_channel().await?;
    let mut marketdata_stream = service.marketdata_stream(channel).await?;

    let request = tinkoff_invest_api::tcs::MarketDataRequest {
        payload: Some(Payload::SubscribeCandlesRequest(SubscribeCandlesRequest {
            subscription_action: SubscriptionAction::Subscribe as i32,
            instruments: vec![
                CandleInstrument {figi:"BBG005P7Q881".to_string(), interval: SubscriptionInterval::OneMinute as i32 },
                CandleInstrument {figi:"BBG00WCNDCZ6".to_string(), interval: SubscriptionInterval::OneMinute as i32 },
            ],
        }))
    };

    let response = marketdata_stream
        .market_data_stream(stream::iter(vec![request]))
        .await?;

    let mut stream = response.into_inner();
    println!("Stream {:?}", stream);

    loop {
        match stream.message().await {
            Ok(packet) => {
                println!("ok {:?}", packet);
            },
            Err(err) => {
                println!("err {}", err);
            },
        };
    }

    Ok(())
}
