# tinkoff-invest-rust

```rust
let service = TinkoffInvestService::new("my_token".to_string());
let channel = service.create_channel().await?;
let mut users = service.users(channel).await?;

let accounts = users
    .get_accounts(tonic::Request::new(
        tinkoff_invest_rust::tcs::GetAccountsRequest {},
    ))
    .await?;

println!("Response {:?}", accounts);
```

# LICENSE

MIT
