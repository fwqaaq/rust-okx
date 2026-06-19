# Contributing

This guide covers how to add a new OKX API endpoint to **rust-okx**.

## Scope: one endpoint group per PR

Each pull request should implement **at most one OKX API endpoint**, or the natural CRUD cluster for a single resource (e.g., place + cancel + amend for the same order type). Larger changes are harder to review and make `TODO.md` bookkeeping error-prone.

---

## File layout

When adding endpoint `GET /api/v5/<module>/<resource>`, touch these files in order:

```text
src/api/<module>/endpoints.rs          add the URL constant
src/api/<module>/requests.rs           add the request type (if needed)
src/api/<module>/responses.rs          add the response type
src/api/<module>/api.rs                add the public method with doc comment
src/api/<module>/tests/<group>.rs      unit test
tests/<module>/read_only.rs            integration test
TODO.md                                mark the endpoint checkbox
```

---

## Doc comment format

Every public method on an accessor type must follow this template exactly.

### Public endpoint (no credentials required)

```rust
/// Retrieve the current funding rate.
///
/// `GET /api/v5/public/funding-rate`. Public. Pass `inst_id` to select the
/// perpetual swap instrument (e.g. `"BTC-USDT-SWAP"`).
///
/// # Errors
///
/// Returns [`Error::Api`] on a non-zero OKX code, or transport/decode errors.
pub async fn get_funding_rate(&self, inst_id: &str) -> Result<Vec<FundingRate>, Error> {}
```

## Response types

Every response struct must carry these attributes:

```rust
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct FundingRate {
    /// Value returned by OKX in the `instId` field.
    #[serde(default)]
    pub inst_id: String,
    /// Value returned by OKX in the `fundingRate` field.
    #[serde(default)]
    pub funding_rate: NumberString,
    /// Value returned by OKX in the `ts` field.
    #[serde(default)]
    pub ts: NumberString,
}
```

- `#[non_exhaustive]` — prevents downstream callers from constructing the struct directly and allows adding fields in a minor release without a semver break.
- `#[serde(default)]` on each field — OKX sometimes omits optional or zero-value fields; `default` prevents a deserialization error when a field is absent.

---

## Unit tests

Unit tests use `MockTransport` and run completely offline. They belong in
`src/api/<module>/tests/<group>.rs`.

### Mock body rule

The `r#"..."#` body string **must be copied from the OKX API documentation
"Response Example" block** for that endpoint. Trim the JSON to the fields your
response type actually declares; do not invent values.

### What to assert

| Concern | Assert |
| --- | --- |
| HTTP method | `assert_eq!(req.method, http::Method::GET)` |
| Endpoint path (no query) | `assert!(req.uri.ends_with("/api/v5/..."))` |
| Query params | `assert_eq!(req.query(), Some("key=val&..."))` |
| POST body field | `let s: Value = from_str(req.body_str()).unwrap(); assert_eq!(s["field"], "value")` |
| Auth signing | `assert!(req.is_signed())` or `assert!(!req.is_signed())` |
| Response field | `assert_eq!(rows[0].some_field.as_str(), "expected")` |
| Validation short-circuits | `assert!(!mock.was_called())` after an `unwrap_err()` |

> When the endpoint has a query string, do **not** use `uri.ends_with(path)` —
> the query suffix would make it fail. Use `req.query()` instead and omit the
> separate path check (or include the full `path?query` string in `ends_with`).

### Worked example — authenticated GET, no validation

```rust
// src/api/account/tests/balance.rs

#[tokio::test]
async fn get_risk_state_signs_and_parses() {
    // Body copied from OKX docs "Response Example" for GET /api/v5/account/risk-state
    let body = r#"{"code":"0","data":[{"debt":"0.85893159114900247077000000000000","interest":"0.00000000000000000000000000000000","loanAlloc":"","nextDiscountTime":"1729490400000","nextInterestTime":"1729490400000","records":[{"availLoan":"","avgRate":"","ccy":"BTC","interest":"0","loanQuota":"175.00000000","posLoan":"","rate":"0.0000276","surplusLmt":"175.00000000","surplusLmtDetails":{},"usedLmt":"0.00000000","usedLoan":"","interestFreeLiab":"","potentialBorrowingAmt":""}]}],"msg":""}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());

    let result = client.account().get_risk_state().await.unwrap();
    assert_eq!(result[0].at_risk, "false");

    let req = mock.captured();
    assert!(req.uri.ends_with("/api/v5/account/risk-state"));
    assert_eq!(req.query(), None);
    assert!(req.is_signed());
}
```

### Worked example — authenticated GET, with validation

```rust
// src/api/account/tests/borrowing.rs

#[tokio::test]
async fn get_max_loan_uses_builder_query() {
    // Body copied from OKX docs "Response Example" for GET /api/v5/account/max-loan
    let body = r#"{"code":"0","msg":"","data":[{"instId":"BTC-USDT","mgnMode":"isolated","mgnCcy":"","maxLoan":"0.1","ccy":"BTC","side":"sell"},{"instId":"BTC-USDT","mgnMode":"isolated","mgnCcy":"USDT","maxLoan":"0.2","ccy":"USDT","side":"buy"}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let request = MaxLoanRequest::new("BTC-USDT", TradeMode::Cross).margin_currency("USDT");

    let result = client.account().get_max_loan(&request).await.unwrap();
    assert_eq!(result[0].max_loan.as_str(), "0.1");

    let req = mock.captured();
    assert_eq!(req.query(), Some("instId=BTC-USDT&mgnMode=cross&mgnCcy=USDT"));
    assert!(req.is_signed());
}
```

---

## Integration tests

Integration tests hit the live OKX API and belong in `tests/<module>/read_only.rs`.
They skip automatically when the required environment variables are absent.

### Credential helpers (from `tests/common/mod.rs`)

| Helper | Use case |
| --- | --- |
| `public_client()` | Unauthenticated (public) endpoints |
| `live_client_or_skip(name)` | Authenticated endpoints; returns `None` → test returns early |
| `expect_ok_or_api_unavailable(result, name)` | Endpoints gated by account tier or region |

### Worked example — public endpoint

```rust
// tests/public_data/core.rs

#[tokio::test]
async fn public_instruments_and_time_parse() {
    let client = public_client();

    // API: GET /api/v5/public/instruments
    let instruments = client
        .public_data()
        .get_instruments(InstType::Spot, None)
        .await
        .expect("public/instruments");
    assert!(instruments.iter().any(|row| row.inst_id == "BTC-USDT"));

    // API: GET /api/v5/public/time
    let rows = client
        .public_data()
        .get_system_time()
        .await
        .expect("public/time");
    assert!(!rows.is_empty());
}
```

### Worked example — authenticated endpoint

```rust
// tests/funding/read_only.rs

#[tokio::test]
async fn funding_currency_and_balance_endpoints_parse() {
    let Some(client) = live_client_or_skip("funding_currency_and_balance_endpoints_parse") else {
        return;
    };

    // API: GET /api/v5/asset/currencies
    let currencies = client
        .funding()
        .get_currencies(Some("USDT"))
        .await
        .expect("asset/currencies");
    assert!(currencies.iter().any(|row| row.ccy == "USDT"));

    // API: GET /api/v5/asset/balances
    let balances = client
        .funding()
        .get_balances(None)
        .await
        .expect("asset/balances");
    assert!(balances.iter().all(|row| !row.ccy.is_empty()));
}
```

### Running integration tests

Copy `.env.example` to `.env` and fill in your credentials, then:

```sh
cargo test                     # offline tests only (no .env required)
cargo test --test funding      # funding integration tests
cargo test --all-features      # all features including websocket
```

---

## PR checklist

Copy this into your pull request description:

## Checklist

- [ ] URL constant added to `src/api/<module>/endpoints.rs`
- [ ] Request type with `ValidateRequest` impl (if applicable)
- [ ] Response type with `#[non_exhaustive]` and `#[serde(default)]` on every field
- [ ] API method doc comment follows the project template (method + path + auth + errors)
- [ ] Unit test mock body copied from OKX API official Response Example
- [ ] Integration test using `live_client_or_skip` or `public_client`
- [ ] `TODO.md` checkbox marked for this endpoint
- [ ] `cargo test --lib` passes
- [ ] `cargo clippy --tests` is clean
