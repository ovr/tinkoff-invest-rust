# tinkoff-invest-rust

> gRPC Клиент для Тинькофф Инвестиций (протокол v2)

[![Crates.io](https://img.shields.io/crates/v/tinkoff-invest-api)](https://crates.io/crates/tinkoff-invest-api)
[![Documentation](https://docs.rs/tinkoff-invest-api/badge.svg)](https://docs.rs/tinkoff-invest-api)
[![Crates.io](https://img.shields.io/crates/l/tinkoff-invest-api)](LICENSE)

## Example

First you need to install:

```toml
[dependencies]
tinkoff-invest-rust = { version = "0.2" }
```

Then, on your main.rs:

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

## License

This project is licensed under the [MIT license](LICENSE).
