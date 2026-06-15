# rust-okx

Async Rust client for the [OKX v5 REST API](https://www.okx.com/docs-v5/en/) with typed models, pluggable transport, and demo trading support.

![MSRV](https://img.shields.io/badge/MSRV-1.85-blue)
![Edition](https://img.shields.io/badge/edition-2024-blue)
![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue)
![Default transport](https://img.shields.io/badge/default%20transport-reqwest-green)

> **Status: early / in development.** `rust-okx` is currently a `0.1.x` crate.
> The core REST client and high-frequency REST endpoints are implemented and
> tested, but full OKX API coverage is still expanding. Expect occasional
> breaking changes before `1.0`.

## Why rust-okx?

- **Typed REST API.** Endpoints are grouped as `market`, `public_data`,
  `account`, and `trade`, with typed request builders and response models.
- **Pluggable transport.** `OkxClient<T>` is generic over a small `Transport`
  trait. You can use the default `reqwest` transport or provide your own mock,
  retrying client, recorder, or test transport.
- **No async trait boxing.** The public transport trait uses return-position
  `impl Future`, so there is no `async_trait` dependency and no required
  `Box<dyn Trait>` dispatch.
- **Lossless numeric values.** OKX sends prices, sizes, and balances as JSON
  strings. `NumberString` preserves the exact wire value and lets callers decide
  whether to parse into `f64`, `Decimal`, or a domain type.
- **Match-friendly errors.** Errors are exposed as a flat enum with transport,
  encoding, decoding, OKX API, HTTP status, and configuration cases.
- **Demo trading support.** `OkxClientBuilder::demo_trading(true)` sends the
  `x-simulated-trading: 1` header required by OKX demo trading.

## Installation

Add the crate and an async runtime to your `Cargo.toml`:

```toml
[dependencies]
rust-okx = "0.1"
tokio = { version = "1", features = ["rt-multi-thread", "macros"] }
```

By default, `rust-okx` enables the built-in `reqwest` transport. Disable default
features when you want to provide a custom transport:

```toml
[dependencies]
rust-okx = { version = "0.1", default-features = false }
```

## Quick start

### Public market data

Public market data does not require credentials:

```rust
use rust_okx::OkxClient;

#[tokio::main]
async fn main() -> Result<(), rust_okx::Error> {
    let client = OkxClient::builder().build();

    let ticker = client.market().get_ticker("BTC-USDT").await?;
    println!("BTC-USDT last price: {}", ticker[0].last.as_str());

    Ok(())
}
```

### Authenticated requests

Authenticated endpoints require an API key, secret, and passphrase:

```rust
use rust_okx::{Credentials, OkxClient};

#[tokio::main]
async fn main() -> Result<(), rust_okx::Error> {
    let credentials = Credentials::new("api-key", "api-secret", "passphrase");
    let client = OkxClient::builder().credentials(credentials).build();

    let balances = client.account().get_balance(None).await?;
    println!("total equity: {}", balances[0].total_eq.as_str());

    Ok(())
}
```

### Demo trading

OKX uses separate API keys for live and demo trading. A live key cannot be used
against the demo environment, and a demo key cannot be used against live trading.

```rust
use rust_okx::{Credentials, OkxClient};

let credentials = Credentials::new("demo-key", "demo-secret", "demo-passphrase");
let client = OkxClient::builder()
    .credentials(credentials)
    .demo_trading(true)
    .build();
```

### Regional accounts

The default client uses the global OKX API domain. Regional accounts must use
the matching domain. US and AU users should select `OkxRegion::Us`; EU users
should select `OkxRegion::Eea`.

If you are not sure which domain applies to your account, check the OKX account
site where you registered and the matching official regional API documentation.

```rust
use rust_okx::{OkxClient, OkxRegion};

let client = OkxClient::builder()
    .region(OkxRegion::Eea)
    .build();
```

## API coverage

Endpoints are reached through accessors on `OkxClient`.

| Accessor | Status | Coverage |
|---|---:|---|
| `client.market()` | Implemented | Tickers, order books, candlesticks, trades, index data, exchange rate, platform volume. |
| `client.public_data()` | Implemented | Instruments, system time, open interest, funding rate, price limit, mark price, delivery history, position tiers, insurance fund. |
| `client.account()` | Implemented | Balance, positions, config, bills, leverage, fees, risk state, simulated margin, position builder, borrowing and account settings. |
| `client.trade()` | Implemented | Place, cancel, amend, close positions, open orders, order history, fills, batch order flows. |
| WebSocket | Not implemented | Planned as a separate public/private streaming API. |
| Funding / Asset | Not implemented | Planned for balances, transfers, deposits, withdrawals, bills, and asset valuation. |
| SubAccount | Not implemented | Planned for sub-account queries, transfers, API keys, and VIP loan allocation. |

See [TODO.md](TODO.md) for the detailed roadmap.

## Design notes

### Transport

The transport layer sends a fully built `http::Request<bytes::Bytes>` and
returns a raw `http::Response<bytes::Bytes>`. Authentication, signing, endpoint
paths, query serialization, JSON encoding, and OKX response envelopes stay in
the client.

```rust
use bytes::Bytes;
use rust_okx::{OkxClient, Transport, TransportError};

#[derive(Clone)]
struct MyTransport;

impl Transport for MyTransport {
    fn send(
        &self,
        request: http::Request<Bytes>,
    ) -> impl std::future::Future<Output = Result<http::Response<Bytes>, TransportError>> + Send
    {
        async move {
            let _ = request;
            todo!("send the request with your HTTP stack")
        }
    }
}

let client = OkxClient::with_transport(MyTransport).build();
```

### Numeric precision

OKX encodes many numeric values as strings. `NumberString` keeps the original
string and provides `as_str()`, `parse::<T>()`, and, with the `rust-decimal`
feature, `to_decimal()`.

### Error handling

The crate exposes a matchable `Error` enum:

- `Transport`
- `Encode`
- `Decode`
- `Api { code, message }`
- `HttpStatus { status, body }`
- `Configuration`

### Feature flags

All feature flags are additive.

| Feature | Default | Effect |
|---|---:|---|
| `reqwest` | Yes | Enables the built-in `ReqwestTransport`. |
| `rust-decimal` | No | Adds `NumberString::to_decimal()`. |

## Testing

The test suite is designed to be useful without network access or credentials.

- Unit tests under `src/**` use an offline mock transport. They assert request
  method, path, query, body, signing headers, response parsing, enum
  compatibility, and error mapping.
- Public integration tests can query public OKX market data.
- Authenticated integration tests load credentials from environment variables
  and skip automatically when the variables are missing.
- Live account tests are read-only.
- Order placement and lifecycle checks run only against demo trading.

Environment variables:

```sh
# Live account, read-only tests
OKX_API_KEY=...
OKX_API_SECRET=...
OKX_PASSPHRASE=...

# Demo trading tests
OKX_DEMO_API_KEY=...
OKX_DEMO_API_SECRET=...
OKX_DEMO_PASSPHRASE=...
```

You may place these values in a `.env` file at the repository root. The tests
load it automatically.

Useful commands:

```sh
cargo test
cargo test --no-default-features --lib
cargo test --all-features
cargo test --test market_public
cargo doc --no-deps
cargo clippy --all-targets --all-features
```

## MSRV and features

- Minimum supported Rust version: `1.85`
- Edition: `2024`
- Default feature: `reqwest`
- Optional feature: `rust-decimal`
- `no_std`: not supported

## Roadmap

The next large areas are WebSocket, Funding / Asset, SubAccount, advanced trade
APIs, and finance modules. See [TODO.md](TODO.md) for the current backlog.

## Disclaimer

This is an unofficial OKX client and is not affiliated with OKX. Trading
involves financial risk. Test your flows against the demo environment before
using live credentials.

## License

Licensed under `MIT OR Apache-2.0`.
