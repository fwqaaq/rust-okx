# OKX Global API Implementation TODO

> Audit snapshot: 2026-06-17
>
> Scope: REST endpoints present in the live OKX Global API guide (`https://www.okx.com/docs-v5/en/`) on 2026-06-17.
> Historical changelog entries, endpoints explicitly marked offline/decommissioned, regional-only catalogs, and announced-but-not-live endpoints are excluded.
> WebSocket channels and WebSocket order operations are intentionally excluded because they use channel/op names rather than HTTP method + path.
>
> Completion rule: an endpoint is checked only when the repository exposes a callable Rust method that sends that exact HTTP method and path. A constant, model, test stub, or WebSocket operation alone does not count.
>
> Progress: **151/342 (44.2%)**

## Account

- [x] GET /api/v5/account/instruments
- [x] GET /api/v5/account/balance
- [x] GET /api/v5/account/positions
- [x] GET /api/v5/account/positions-history
- [x] GET /api/v5/account/account-position-risk
- [x] GET /api/v5/account/bills
- [x] GET /api/v5/account/bills-archive
- [ ] POST /api/v5/account/bills-history-archive
- [ ] GET /api/v5/account/bills-history-archive
- [ ] GET /api/v5/account/subtypes
- [x] GET /api/v5/account/config
- [x] POST /api/v5/account/set-position-mode
- [x] POST /api/v5/account/set-leverage
- [x] GET /api/v5/account/max-size
- [x] GET /api/v5/account/max-avail-size
- [x] POST /api/v5/account/position/margin-balance
- [x] GET /api/v5/account/leverage-info
- [ ] GET /api/v5/account/adjust-leverage-info
- [x] GET /api/v5/account/max-loan
- [x] GET /api/v5/account/trade-fee
- [x] GET /api/v5/account/interest-accrued
- [x] GET /api/v5/account/interest-rate
- [x] GET /api/v5/account/max-withdrawal
- [x] GET /api/v5/account/interest-limits
- [x] POST /api/v5/account/spot-manual-borrow-repay
- [x] POST /api/v5/account/set-auto-repay
- [x] GET /api/v5/account/spot-borrow-repay-history
- [x] POST /api/v5/account/set-auto-earn
- [x] POST /api/v5/account/set-greeks
- [x] GET /api/v5/account/greeks
- [x] POST /api/v5/account/set-isolated-mode
- [x] POST /api/v5/account/simulated_margin
- [x] GET /api/v5/account/position-tiers
- [x] GET /api/v5/account/risk-state
- [x] POST /api/v5/account/set-riskOffset-type
- [ ] POST /api/v5/account/set-riskOffset-amt
- [x] POST /api/v5/account/set-auto-loan
- [x] POST /api/v5/account/set-account-level
- [x] POST /api/v5/account/activate-option
- [ ] POST /api/v5/account/set-collateral-assets
- [ ] GET /api/v5/account/collateral-assets
- [ ] GET /api/v5/account/precheck-set-delta-neutral
- [x] POST /api/v5/account/position-builder
- [ ] POST /api/v5/account/move-positions
- [ ] GET /api/v5/account/move-positions-history
- [ ] POST /api/v5/account/set-settle-currency
- [ ] POST /api/v5/account/set-fee-type
- [ ] GET /api/v5/account/mmp-config
- [ ] POST /api/v5/account/mmp-config
- [ ] POST /api/v5/account/mmp-reset
- [ ] POST /api/v5/account/demo-adjust-balance

<!-- Account: 35/51 implemented -->

## Trade

- [x] POST /api/v5/trade/order
- [x] POST /api/v5/trade/batch-orders
- [x] POST /api/v5/trade/cancel-order
- [x] POST /api/v5/trade/cancel-batch-orders
- [x] POST /api/v5/trade/amend-order
- [x] POST /api/v5/trade/amend-batch-orders
- [x] POST /api/v5/trade/close-position
- [x] GET /api/v5/trade/order
- [x] GET /api/v5/trade/orders-pending
- [x] GET /api/v5/trade/orders-history
- [x] GET /api/v5/trade/orders-history-archive
- [x] GET /api/v5/trade/fills
- [x] GET /api/v5/trade/fills-history
- [ ] POST /api/v5/trade/cancel-all-after
- [ ] GET /api/v5/trade/account-rate-limit
- [ ] POST /api/v5/trade/order-precheck
- [ ] POST /api/v5/trade/mass-cancel
- [x] GET /api/v5/trade/easy-convert-currency-list
- [x] POST /api/v5/trade/easy-convert
- [x] GET /api/v5/trade/easy-convert-history
- [x] GET /api/v5/trade/one-click-repay-currency-list
- [x] POST /api/v5/trade/one-click-repay
- [x] GET /api/v5/trade/one-click-repay-history
- [x] GET /api/v5/trade/one-click-repay-currency-list-v2
- [x] POST /api/v5/trade/one-click-repay-v2
- [x] GET /api/v5/trade/one-click-repay-history-v2

<!-- Trade: 22/26 implemented -->

## Algo Trading

- [x] POST /api/v5/trade/order-algo
- [x] POST /api/v5/trade/cancel-algos
- [x] POST /api/v5/trade/amend-algos
- [x] GET /api/v5/trade/order-algo
- [x] GET /api/v5/trade/orders-algo-pending
- [x] GET /api/v5/trade/orders-algo-history

<!-- Algo Trading: 6/6 implemented -->

## Market Data

- [x] GET /api/v5/market/tickers
- [x] GET /api/v5/market/ticker
- [x] GET /api/v5/market/index-tickers
- [x] GET /api/v5/market/books
- [ ] GET /api/v5/market/books-full
- [x] GET /api/v5/market/candles
- [x] GET /api/v5/market/history-candles
- [x] GET /api/v5/market/index-candles
- [ ] GET /api/v5/market/history-index-candles
- [x] GET /api/v5/market/mark-price-candles
- [ ] GET /api/v5/market/history-mark-price-candles
- [x] GET /api/v5/market/trades
- [x] GET /api/v5/market/history-trades
- [x] GET /api/v5/market/platform-24-volume
- [x] GET /api/v5/market/index-components
- [x] GET /api/v5/market/exchange-rate
- [ ] GET /api/v5/market/open-oracle
- [x] GET /api/v5/market/block-ticker
- [x] GET /api/v5/market/block-tickers
- [x] GET /api/v5/market/option/instrument-family-trades
- [ ] GET /api/v5/market/call-auction-details

<!-- Market Data: 16/21 implemented -->

## Public Data

- [x] GET /api/v5/public/instruments
- [x] GET /api/v5/public/time
- [x] GET /api/v5/public/open-interest
- [x] GET /api/v5/public/funding-rate
- [x] GET /api/v5/public/funding-rate-history
- [x] GET /api/v5/public/price-limit
- [x] GET /api/v5/public/mark-price
- [x] GET /api/v5/public/estimated-price
- [x] GET /api/v5/public/delivery-exercise-history
- [x] GET /api/v5/public/position-tiers
- [x] GET /api/v5/public/interest-rate-loan-quota
- [x] GET /api/v5/public/underlying
- [x] GET /api/v5/public/insurance-fund
- [x] GET /api/v5/public/opt-summary
- [x] GET /api/v5/public/convert-contract-coin
- [x] GET /api/v5/public/discount-rate-interest-free-quota
- [x] GET /api/v5/public/instrument-tick-bands
- [x] GET /api/v5/public/option-trades
- [x] GET /api/v5/public/market-data-history
- [ ] GET /api/v5/public/economic-calendar
- [ ] GET /api/v5/public/premium-history
- [ ] GET /api/v5/public/event-contract/series
- [ ] GET /api/v5/public/event-contract/events
- [ ] GET /api/v5/public/event-contract/markets

<!-- Public Data: 19/25 implemented -->

## Trading Data

- [ ] GET /api/v5/rubik/stat/trading-data/support-coin
- [ ] GET /api/v5/rubik/stat/taker-volume
- [ ] GET /api/v5/rubik/stat/margin/loan-ratio
- [ ] GET /api/v5/rubik/stat/contracts/long-short-account-ratio
- [ ] GET /api/v5/rubik/stat/contracts/long-short-account-ratio-contract
- [ ] GET /api/v5/rubik/stat/contracts/open-interest-volume
- [ ] GET /api/v5/rubik/stat/contracts/open-interest-history
- [ ] GET /api/v5/rubik/stat/option/open-interest-volume
- [ ] GET /api/v5/rubik/stat/option/open-interest-volume-ratio
- [ ] GET /api/v5/rubik/stat/option/open-interest-volume-expiry
- [ ] GET /api/v5/rubik/stat/option/open-interest-volume-strike
- [ ] GET /api/v5/rubik/stat/option/taker-block-volume

<!-- Trading Data: 0/12 implemented -->

## Funding Account

- [x] GET /api/v5/asset/currencies
- [x] GET /api/v5/asset/balances
- [x] GET /api/v5/asset/non-tradable-assets
- [x] GET /api/v5/asset/asset-valuation
- [x] POST /api/v5/asset/transfer
- [x] GET /api/v5/asset/transfer-state
- [x] GET /api/v5/asset/bills
- [ ] GET /api/v5/asset/bills-history
- [x] GET /api/v5/asset/deposit-address
- [x] GET /api/v5/asset/deposit-history
- [x] POST /api/v5/asset/withdrawal
- [x] POST /api/v5/asset/cancel-withdrawal
- [x] GET /api/v5/asset/withdrawal-history
- [x] GET /api/v5/asset/deposit-withdraw-status
- [ ] GET /api/v5/asset/exchange-list
- [ ] POST /api/v5/asset/monthly-statement
- [ ] GET /api/v5/asset/monthly-statement
- [x] GET /api/v5/asset/deposit-lightning
- [x] POST /api/v5/asset/withdrawal-lightning

<!-- Funding Account: 15/19 implemented -->

## Convert

- [x] GET /api/v5/asset/convert/currencies
- [x] GET /api/v5/asset/convert/currency-pair
- [x] POST /api/v5/asset/convert/estimate-quote
- [x] POST /api/v5/asset/convert/trade
- [x] GET /api/v5/asset/convert/history

<!-- Convert: 5/5 implemented -->

## Fiat

- [ ] GET /api/v5/fiat/deposit-payment-methods
- [ ] GET /api/v5/fiat/withdrawal-payment-methods
- [ ] POST /api/v5/fiat/create-withdrawal
- [ ] POST /api/v5/fiat/cancel-withdrawal
- [ ] GET /api/v5/fiat/withdrawal-order-history
- [ ] GET /api/v5/fiat/withdrawal
- [ ] GET /api/v5/fiat/deposit-order-history
- [ ] GET /api/v5/fiat/deposit
- [ ] GET /api/v5/fiat/buy-sell/currencies
- [ ] GET /api/v5/fiat/buy-sell/currency-pair
- [ ] POST /api/v5/fiat/buy-sell/quote
- [ ] POST /api/v5/fiat/buy-sell/trade
- [ ] GET /api/v5/fiat/buy-sell/history

<!-- Fiat: 0/13 implemented -->

## Sub-account

- [ ] GET /api/v5/users/subaccount/list
- [ ] POST /api/v5/users/subaccount/create-subaccount
- [ ] POST /api/v5/users/subaccount/apikey
- [ ] GET /api/v5/users/subaccount/apikey
- [ ] POST /api/v5/users/subaccount/modify-apikey
- [ ] POST /api/v5/users/subaccount/delete-apikey
- [ ] GET /api/v5/account/subaccount/balances
- [ ] GET /api/v5/asset/subaccount/balances
- [ ] GET /api/v5/account/subaccount/max-withdrawal
- [ ] GET /api/v5/asset/subaccount/bills
- [ ] GET /api/v5/asset/subaccount/managed-subaccount-bills
- [ ] POST /api/v5/asset/subaccount/transfer
- [ ] POST /api/v5/users/subaccount/set-transfer-out
- [ ] GET /api/v5/users/entrust-subaccount-list
- [ ] POST /api/v5/account/subaccount/set-loan-allocation
- [ ] GET /api/v5/account/subaccount/interest-limits

<!-- Sub-account: 0/16 implemented -->

## Financial Product — On-chain Earn

- [x] GET /api/v5/finance/staking-defi/offers
- [x] POST /api/v5/finance/staking-defi/purchase
- [x] POST /api/v5/finance/staking-defi/redeem
- [x] POST /api/v5/finance/staking-defi/cancel
- [x] GET /api/v5/finance/staking-defi/orders-active
- [x] GET /api/v5/finance/staking-defi/orders-history

<!-- Financial Product — On-chain Earn: 6/6 implemented -->

## Financial Product — Simple Earn Flexible

- [x] GET /api/v5/finance/savings/balance
- [x] POST /api/v5/finance/savings/purchase-redempt
- [x] POST /api/v5/finance/savings/set-lending-rate
- [x] GET /api/v5/finance/savings/lending-history
- [x] GET /api/v5/finance/savings/lending-rate-summary
- [x] GET /api/v5/finance/savings/lending-rate-history

<!-- Financial Product — Simple Earn Flexible: 6/6 implemented -->

## Financial Product — ETH Staking

- [x] GET /api/v5/finance/staking-defi/eth/product-info
- [x] POST /api/v5/finance/staking-defi/eth/purchase
- [x] POST /api/v5/finance/staking-defi/eth/redeem
- [x] POST /api/v5/finance/staking-defi/eth/cancel-redeem
- [x] GET /api/v5/finance/staking-defi/eth/balance
- [x] GET /api/v5/finance/staking-defi/eth/purchase-redeem-history
- [x] GET /api/v5/finance/staking-defi/eth/apy-history

<!-- Financial Product — ETH Staking: 7/7 implemented -->

## Financial Product — SOL Staking

- [x] GET /api/v5/finance/staking-defi/sol/product-info
- [x] POST /api/v5/finance/staking-defi/sol/purchase
- [x] POST /api/v5/finance/staking-defi/sol/redeem
- [x] GET /api/v5/finance/staking-defi/sol/balance
- [x] GET /api/v5/finance/staking-defi/sol/purchase-redeem-history
- [x] GET /api/v5/finance/staking-defi/sol/apy-history

<!-- Financial Product — SOL Staking: 6/6 implemented -->

## Financial Product — Dual Investment

- [ ] GET /api/v5/finance/sfp/dcd/currency-pair
- [ ] GET /api/v5/finance/sfp/dcd/products
- [ ] POST /api/v5/finance/sfp/dcd/quote
- [ ] POST /api/v5/finance/sfp/dcd/trade
- [ ] POST /api/v5/finance/sfp/dcd/redeem-quote
- [ ] POST /api/v5/finance/sfp/dcd/redeem
- [ ] GET /api/v5/finance/sfp/dcd/order-status
- [ ] GET /api/v5/finance/sfp/dcd/order-history

<!-- Financial Product — Dual Investment: 0/8 implemented -->

## Financial Product — Stable Rewards

- [ ] GET /api/v5/finance/stable-rewards/product-info
- [ ] POST /api/v5/finance/stable-rewards/quote
- [ ] POST /api/v5/finance/stable-rewards/trade
- [ ] GET /api/v5/finance/stable-rewards/balance
- [ ] GET /api/v5/finance/stable-rewards/apy-history
- [ ] GET /api/v5/finance/stable-rewards/subscribe-redeem-history

<!-- Financial Product — Stable Rewards: 0/6 implemented -->

## Loan — Flexible Loan

- [x] GET /api/v5/finance/flexible-loan/borrow-currencies
- [x] GET /api/v5/finance/flexible-loan/collateral-assets
- [x] POST /api/v5/finance/flexible-loan/max-loan
- [x] GET /api/v5/finance/flexible-loan/max-collateral-redeem-amount
- [x] POST /api/v5/finance/flexible-loan/adjust-collateral
- [x] GET /api/v5/finance/flexible-loan/loan-info
- [x] GET /api/v5/finance/flexible-loan/loan-history
- [x] GET /api/v5/finance/flexible-loan/interest-accrued

<!-- Loan — Flexible Loan: 8/8 implemented -->

## Block Trading / RFQ

- [ ] GET /api/v5/rfq/counterparties
- [ ] POST /api/v5/rfq/create-rfq
- [ ] POST /api/v5/rfq/cancel-rfq
- [ ] POST /api/v5/rfq/cancel-batch-rfqs
- [ ] POST /api/v5/rfq/cancel-all-rfqs
- [ ] POST /api/v5/rfq/execute-quote
- [ ] GET /api/v5/rfq/quote-products
- [ ] POST /api/v5/rfq/set-quote-products
- [ ] POST /api/v5/rfq/create-quote
- [ ] POST /api/v5/rfq/cancel-quote
- [ ] POST /api/v5/rfq/cancel-batch-quotes
- [ ] POST /api/v5/rfq/cancel-all-quotes
- [ ] GET /api/v5/rfq/rfqs
- [ ] GET /api/v5/rfq/quotes
- [ ] GET /api/v5/rfq/trades
- [ ] GET /api/v5/rfq/public-trades
- [ ] GET /api/v5/rfq/maker-instrument-settings
- [ ] POST /api/v5/rfq/maker-instrument-settings
- [ ] GET /api/v5/rfq/mmp-config
- [ ] POST /api/v5/rfq/mmp-config
- [ ] POST /api/v5/rfq/mmp-reset

<!-- Block Trading / RFQ: 0/21 implemented -->

## Spread Trading

- [ ] GET /api/v5/sprd/spreads
- [ ] GET /api/v5/sprd/books
- [ ] GET /api/v5/sprd/ticker
- [ ] GET /api/v5/sprd/public-trades
- [ ] GET /api/v5/sprd/candles
- [ ] GET /api/v5/sprd/history-candles
- [ ] POST /api/v5/sprd/order
- [ ] POST /api/v5/sprd/cancel-order
- [ ] POST /api/v5/sprd/mass-cancel
- [ ] POST /api/v5/sprd/amend-order
- [ ] GET /api/v5/sprd/order
- [ ] GET /api/v5/sprd/orders-pending
- [ ] GET /api/v5/sprd/orders-history
- [ ] GET /api/v5/sprd/orders-history-archive
- [ ] GET /api/v5/sprd/trades

<!-- Spread Trading: 0/15 implemented -->

## Trading Bot — Grid

- [ ] POST /api/v5/tradingBot/grid/order-algo
- [ ] POST /api/v5/tradingBot/grid/amend-order-algo
- [ ] POST /api/v5/tradingBot/grid/stop-order-algo
- [ ] GET /api/v5/tradingBot/grid/orders-algo-pending
- [ ] GET /api/v5/tradingBot/grid/orders-algo-history
- [ ] GET /api/v5/tradingBot/grid/orders-algo-details
- [ ] GET /api/v5/tradingBot/grid/sub-orders
- [ ] GET /api/v5/tradingBot/grid/positions
- [ ] POST /api/v5/tradingBot/grid/close-position
- [ ] POST /api/v5/tradingBot/grid/cancel-close-order
- [ ] POST /api/v5/tradingBot/grid/withdraw-income
- [ ] POST /api/v5/tradingBot/grid/compute-margin-balance
- [ ] POST /api/v5/tradingBot/grid/margin-balance
- [ ] GET /api/v5/tradingBot/grid/ai-param
- [ ] POST /api/v5/tradingBot/grid/min-investment
- [ ] POST /api/v5/tradingBot/grid/adjust-investment
- [ ] POST /api/v5/tradingBot/grid/order-instant-trigger
- [ ] POST /api/v5/tradingBot/grid/amend-order-instant-trigger
- [ ] POST /api/v5/tradingBot/grid/stop-order-instant-trigger

<!-- Trading Bot — Grid: 0/19 implemented -->

## Trading Bot — Recurring Buy

- [ ] POST /api/v5/tradingBot/recurring/order-algo
- [ ] POST /api/v5/tradingBot/recurring/amend-order-algo
- [ ] POST /api/v5/tradingBot/recurring/stop-order-algo
- [ ] GET /api/v5/tradingBot/recurring/orders-algo-pending
- [ ] GET /api/v5/tradingBot/recurring/orders-algo-history
- [ ] GET /api/v5/tradingBot/recurring/orders-algo-details
- [ ] GET /api/v5/tradingBot/recurring/sub-orders

<!-- Trading Bot — Recurring Buy: 0/7 implemented -->

## Trading Bot — Signal

- [ ] POST /api/v5/tradingBot/signal/create-signal
- [ ] GET /api/v5/tradingBot/signal/signals
- [ ] POST /api/v5/tradingBot/signal/order-algo
- [ ] POST /api/v5/tradingBot/signal/stop-order-algo
- [ ] GET /api/v5/tradingBot/signal/orders-algo-details
- [ ] GET /api/v5/tradingBot/signal/orders-algo-pending
- [ ] GET /api/v5/tradingBot/signal/orders-algo-history
- [ ] GET /api/v5/tradingBot/signal/sub-orders
- [ ] POST /api/v5/tradingBot/signal/cancel-sub-order

<!-- Trading Bot — Signal: 0/9 implemented -->

## Copy Trading

- [ ] GET /api/v5/copytrading/current-subpositions
- [ ] GET /api/v5/copytrading/subpositions-history
- [ ] POST /api/v5/copytrading/algo-order
- [ ] POST /api/v5/copytrading/close-subposition
- [ ] GET /api/v5/copytrading/instruments
- [ ] POST /api/v5/copytrading/set-instruments
- [ ] GET /api/v5/copytrading/profit-sharing-details
- [ ] GET /api/v5/copytrading/total-profit-sharing
- [ ] GET /api/v5/copytrading/unrealized-profit-sharing-details
- [ ] POST /api/v5/copytrading/stop-copy-trading
- [ ] POST /api/v5/copytrading/batch-set-leverage
- [ ] GET /api/v5/copytrading/current-lead-traders
- [ ] GET /api/v5/copytrading/public-lead-traders
- [ ] GET /api/v5/copytrading/public-stats
- [ ] GET /api/v5/copytrading/public-preference-currency
- [ ] GET /api/v5/copytrading/public-current-subpositions
- [ ] GET /api/v5/copytrading/public-subpositions-history
- [ ] POST /api/v5/copytrade/create-sgl-link

<!-- Copy Trading: 0/18 implemented -->

## Affiliate

- [ ] GET /api/v5/affiliate/performance/summary
- [ ] GET /api/v5/affiliate/invitee/detail
- [ ] GET /api/v5/affiliate/invitee/list
- [ ] GET /api/v5/affiliate/link/list
- [ ] GET /api/v5/affiliate/co-inviter/list
- [ ] GET /api/v5/affiliate/sub-affiliate/list
- [ ] GET /api/v5/users/partner/if-rebate

<!-- Affiliate: 0/7 implemented -->

## Broker / Partner

- [ ] GET /api/v5/broker/fd/rebate-per-orders
- [ ] POST /api/v5/broker/fd/rebate-per-orders

<!-- Broker / Partner: 0/2 implemented -->

## Status

- [ ] GET /api/v5/system/status

<!-- Status: 0/1 implemented -->

## Announcement

- [ ] GET /api/v5/support/announcements
- [ ] GET /api/v5/support/announcement-types

<!-- Announcement: 0/2 implemented -->
