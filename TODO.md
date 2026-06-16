# rust-okx 后续 TODO

本文件记录第三轮之后尚未移植或尚未完整设计的 OKX v5 API 模块。后续实现仍保持当前约束：每个 API 都有离线测试和 DOC 注释，不新增不必要的 public trait，不暴露 `reqwest`、`tokio`、`serde_json` 等内部依赖类型。

## WebSocket

- Round 4 已完成首版 `websocket` feature：`OkxWs` / `OkxWsBuilder` / `WsConn` / `WsConnector` / `WsEvent` / `Arg`。
- 已实现 public/private/business 入口、connect/login/subscribe/unsubscribe/close、基础断线重连、订阅恢复、私有重登录、文本 ping/pong。
- 已实现默认 `tokio-tungstenite` 连接器，且公共 API 不暴露 `tokio` / `tokio_tungstenite` / `serde_json` 类型。
- 已增加离线 Mock 连接测试，以及真实 `ws_public` / `ws_business` / `ws_private` 集成测试。
- 后续继续扩展 public channels：trades、books、books5、candles、instruments、open-interest、funding-rate、price-limit、mark-price 等 typed examples。
- 后续继续扩展 private channels：positions、balance_and_position、orders-algo、algo-advance、liquidation-warning、account-greeks 等 typed models。
- 后续增强重连策略：可配置指数退避、jitter、可注入 sleep、完整 idle timeout 策略和更精细的重连事件。

## Funding / Asset

- Round 5 已完成首版 `client.funding()`：覆盖 Python SDK `FundingAPI` 中的 `/api/v5/asset/*` 端点。
- 已实现资金账户查询：currencies、balances、non-tradable assets、deposit address、deposit history、withdrawal history、funding bills、asset valuation、deposit/withdraw status。
- 已实现资产操作：funds transfer、transfer state、withdrawal、cancel withdrawal、purchase/redempt、convert dust assets、Lightning deposit / withdrawal。
- 已增加 `.env` 驱动的真实集成测试；mutating 端点需要 `OKX_ENABLE_ASSET_MUTATION=1` 和对应参数才执行。
- 后续继续补 SubAccount asset endpoints：subaccount balances、subaccount transfer、transfer state、transfer history。
- 后续继续补 Finance/Savings/Staking/FlexibleLoan 模块，避免和 Funding 模块混用。
- 后续可增强 live lifecycle 覆盖：transfer -> transfer-state、withdrawal -> cancel-withdrawal、deposit/withdraw status 回查。

## SubAccount

- 新增 `sub_account` 模块。
- 子账户查询：子账户列表、子账户交易账户余额、子账户资金账户余额、子账户账单。
- 子账户划转：master/sub transfer、transfer history、transfer state。
- API key 管理：创建、修改、删除、查看子账户 API key。
- 权限设置：转出权限、托管子账户、子账户充值地址。
- VIP loan allocation：借贷额度分配、历史和状态查询。
- 测试重点覆盖 master credentials 签名、query/body 序列化和分页请求 builder。

## Trade 高级能力

- Round 6 已完成首版高级 Trade REST 覆盖。
- 已实现 algo order：下单、撤单、改单、pending list、history、details。
- 已实现 Easy Convert：小额兑换币种列表、兑换执行、历史查询。
- 已实现 One-click Repay v1/v2：支持币种列表、执行还款、历史查询。
- 当前 Rust API 保持普通订单 API 与 algo API 的类型分离，避免单个请求类型膨胀。
- 后续继续增强高级订单 typed builders：conditional、oco、trigger、move_order_stop、twap、iceberg 的字段级建模和 live lifecycle 覆盖。

## PublicData / Market 边角能力

- Round 6 已完成首版 PublicData / Market 边角 REST 覆盖。
- PublicData 已实现：option summary、estimated price、discount quota、interest loan quota、VIP loan quota、option tick bands、option trades、market data history。
- Market 已实现：block ticker、block tickers、block trades、option instrument-family trades。
- 后续继续使用 `NumberString` 保存数值字符串，新增枚举必须保留 `Unknown(String)`。
- 后续可继续把当前 `RestRow` 长尾响应升级为更细 typed models，但应保持非 breaking 扩展。

## Convert / Finance 扩展模块

- Round 6 已完成首版 `client.convert()` 和 `client.finance()`。
- Convert 已实现：currencies、currency-pair、estimate-quote、trade、history。
- Finance 已实现分组 accessor：
  - `finance().savings()`：balance、purchase/redemption、set lending rate、lending history、public borrow info/history。
  - `finance().staking_defi()`：offers、purchase、redeem、cancel、active orders、orders history。
  - `finance().eth_staking()`：product info、purchase、redeem、balance、purchase/redeem history、APY history。
  - `finance().sol_staking()`：product info、purchase、redeem、balance、purchase/redeem history、APY history。
  - `finance().flexible_loan()`：borrow currencies、collateral assets、max loan、max collateral redeem amount、adjust collateral、loan info/history、interest accrued。
- 后续继续补 gated live lifecycle 测试：convert quote -> trade、savings purchase -> redeem、staking purchase -> cancel/redeem、flexible loan borrow -> adjust collateral -> repay。

## 低优先级交易扩展

- SpreadTrading：spread instruments、order、cancel、amend、orders、fills。
- BlockTrading / RFQ：counterparties、rfq、quote、execute quote、mmp。
- GridTrading：网格策略创建、停止、查询、历史和收益。
- CopyTrading：带单、跟单、配置、收益和历史。
- FDBroker：broker rebate、用户映射、佣金和关系查询。
- TradingData：成交量、持仓量、多空比、主动买卖、杠杆借贷比等数据接口。
- Status：系统状态、维护窗口和公告类接口。
