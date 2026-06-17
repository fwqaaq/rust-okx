# Integration-test layout

REST integration tests are grouped by OKX API family:

- `account.rs` + `account/`
- `convert.rs` + `convert/`
- `finance.rs` + `finance/`
- `funding.rs` + `funding/`
- `market.rs` + `market/`
- `public_data.rs` + `public_data/`
- `trade.rs` + `trade/`

Each endpoint annotation uses two lines:

```text
// API: GET /api/v5/example
// STATUS: LIVE — public/read-only and exercised by this test.
```

Supported status prefixes:

- `LIVE`: exercised against the real API; credential-dependent cases skip when credentials are absent.
- `DEMO`: an asset/account mutation exercised only with OKX simulated-trading credentials.
- `TODO`: deliberately not executed; the status must explain why, and the placeholder test uses `#[ignore]` plus `todo!()`.

`api_coverage.rs` verifies that every REST method/path currently implemented by the crate has one of these annotations. Unit tests continue to use mock transports; integration tests do not.
