mod tests {
    use std::env;

    use tinkoff_invest_rust::{TinkoffInvestService, TIResult};

    #[tokio::test]
    async fn test() -> TIResult<()> {
        let service = TinkoffInvestService::new(env::var("TINKOFF_SDK_TESTING_ENV").unwrap());
        let channel = service.create_channel().await?;
        let mut users = service.users(channel).await?;

        let accounts = users
            .get_accounts(tonic::Request::new(
                tinkoff_invest_rust::tcs::GetAccountsRequest {},
            ))
            .await?;

        println!("Response {:?}", accounts);

        Ok(())
    }
}
