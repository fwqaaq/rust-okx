# Request model audit

This document tracks the migration from generic `RequestParams` payloads to
endpoint-specific request models. It is intentionally incremental: a module is
marked complete only after its request fields, wire names, requiredness,
parameter location, validation, and serialization tests have been checked
against the current OKX v5 documentation.

## Status legend

- `TYPED`: endpoint has a dedicated request type.
- `VALIDATED`: documented constraints represented by this SDK are checked before I/O.
- `CONTRACT_TESTED`: method, path, query/body placement, and wire serialization are tested.
- `GENERIC_REQUEST`: endpoint still accepts `RequestParams`/`RawRequestParams`.
- `PENDING_AUDIT`: not yet checked field-by-field against the OKX documentation.

## Convert — first migration slice

| Endpoint | Method | Parameter location | Request model | Status |
| --- | --- | --- | --- | --- |
| `/api/v5/asset/convert/currencies` | GET | none | `ConvertCurrenciesRequest` | TYPED, CONTRACT_TESTED |
| `/api/v5/asset/convert/currency-pair` | GET | query | `ConvertCurrencyPairRequest` | TYPED, VALIDATED, CONTRACT_TESTED |
| `/api/v5/asset/convert/estimate-quote` | POST | JSON body | `ConvertQuoteRequest` | TYPED, VALIDATED, CONTRACT_TESTED |
| `/api/v5/asset/convert/trade` | POST | JSON body | `ConvertTradeRequest` | TYPED, VALIDATED, CONTRACT_TESTED |
| `/api/v5/asset/convert/history` | GET | query | `ConvertHistoryRequest` | TYPED, VALIDATED, CONTRACT_TESTED |

### Convert checks represented in code

- Required currency, quote, size, and quote-ID fields cannot be empty.
- Convert side is restricted to `buy` or `sell`.
- `clQReqId` and `clTReqId` are limited to 32 ASCII alphanumeric characters.
- History `limit` is serialized as an OKX string and constrained to `1..=100`.
- Optional values are omitted rather than serialized as `null`.
- `convertMode` is serialized as `"0"` or `"1"`.
- GET parameters are encoded in the query string; POST parameters are JSON bodies.

### Deliberately deferred Convert checks

The first migration does not attempt to validate decimal syntax, account
balances, quote expiry, whether `sz` exceeds the RFQ amount, broker-tag policy,
or VIP eligibility. Those rules either require server state or need a shared
numeric-string policy before being enforced consistently across modules.

## Shared request infrastructure

`RawRequestParams` remains available as an escape hatch for new or unsupported
OKX fields. `RequestParams` is retained as a compatibility alias. Duplicate
keys now replace the prior value instead of producing duplicate JSON object
members.

Typed request models implement `ValidateRequest`; endpoint accessors return
`Error::InvalidRequest` before transport when validation fails.

## Remaining modules

| Module | Current migration status | Next priority |
| --- | --- | --- |
| Trade | PENDING_AUDIT | Place/amend/cancel order and algo-order mutation bodies |
| Funding | PENDING_AUDIT | Transfer, withdrawal, and cancellation bodies |
| Account | PENDING_AUDIT | Leverage, margin, borrowing, and position-affecting requests |
| Finance | PENDING_AUDIT | Replace generic Savings, Staking/DeFi, and loan parameters |
| Market | PENDING_AUDIT | Query requiredness, limits, cursors, and bar values |
| Public data | PENDING_AUDIT | Split large request file and verify endpoint-specific filters |
| WebSocket | Separate audit | Subscription args, operation payloads, and event envelopes |

## Review checklist for each later endpoint

1. Verify HTTP method and exact path.
2. Verify query versus JSON-body placement.
3. Verify every OKX wire name and serialized primitive type.
4. Make required fields constructor arguments.
5. Omit absent optional fields.
6. Model closed string choices with enums where practical.
7. Add local validation for documented static constraints.
8. Add an official-example serialization test.
9. Add a mock-transport contract test.
10. Keep a clearly named raw escape hatch only when needed.
