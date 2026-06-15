# rust-okx 后续 TODO

本文件记录第三轮之后尚未移植或尚未完整设计的 OKX v5 API 模块。后续实现仍保持当前约束：每个 API 都有离线测试和 DOC 注释，不新增不必要的 public trait，不暴露 `reqwest`、`tokio`、`serde_json` 等内部依赖类型。

## WebSocket

- Round 4 已完成首版 `websocket` feature：`OkxWs` / `OkxWsBuilder` / `WsConn` / `WsConnector` / `WsEvent` / `Arg`。
- 已实现 public/private/business 入口、connect/login/subscribe/unsubscribe/close、基础断线重连、订阅恢复、私有重登录、文本 ping/pong。
- 已实现默认 `tokio-tungstenite` 连接器，且公共 API 不暴露 `tokio` / `tokio_tungstenite` / `serde_json` 类型。
- 已增加离线 Mock 连接测试，以及真实 `ws_public` / `ws_private` 集成测试。
- 后续继续扩展 public channels：trades、books、books5、candles、instruments、open-interest、funding-rate、price-limit、mark-price 等 typed examples。
- 后续继续扩展 private channels：positions、balance_and_position、orders-algo、algo-advance、liquidation-warning、account-greeks 等 typed models。
- 后续增强重连策略：可配置指数退避、jitter、可注入 sleep、完整 idle timeout 策略和更精细的重连事件。

## Funding / Asset

- 新增 `funding` 或 `asset` 模块，覆盖资金账户余额和币种信息。
- 资金划转：funds transfer、transfer state、资金划转历史、账户间划转场景。
- 充值：充值地址、充值历史、充值到账状态。
- 提现：提现、取消提现、提现历史、链上手续费、地址校验。
- 闪电网络：Lightning deposit、Lightning withdraw、invoice 查询。
- 资产估值和资金账单：asset valuation、funding bills、asset bills。
- 保持 Funding 与 Account 的模型边界清晰，避免把资金账户类型混入交易账户 API。

## SubAccount

- 新增 `sub_account` 模块。
- 子账户查询：子账户列表、子账户交易账户余额、子账户资金账户余额、子账户账单。
- 子账户划转：master/sub transfer、transfer history、transfer state。
- API key 管理：创建、修改、删除、查看子账户 API key。
- 权限设置：转出权限、托管子账户、子账户充值地址。
- VIP loan allocation：借贷额度分配、历史和状态查询。
- 测试重点覆盖 master credentials 签名、query/body 序列化和分页请求 builder。

## Trade 高级能力

- Algo order：下单、撤单、改单、批量撤单。
- Algo 查询：未完成 algo list、历史、详情。
- 高级订单类型：conditional、oco、trigger、move_order_stop、twap、iceberg。
- Easy convert：小额兑换列表、兑换执行、历史查询。
- One-click repay：v1/v2 还款、还款历史和状态。
- 保持普通订单 API 与 algo API 的类型分离，避免单个请求类型膨胀。

## PublicData / Market 边角能力

- PublicData：option summary、estimated price、discount quota、interest loan quota、VIP loan quota、option tick bands。
- Market：option trades、market data history、block ticker、block tickers、block trades。
- 继续使用 `NumberString` 保存数值字符串，新增枚举必须保留 `Unknown(String)`。

## Convert / Finance 扩展模块

- Convert：询价、兑换、历史、币对和估价。
- Savings：余额、申购、赎回、利率和历史。
- StakingDefi：产品列表、申购、赎回、订单和收益。
- EthStaking / SolStaking：质押、赎回、余额、订单历史。
- FlexibleLoan：借款、还款、抵押品调整、订单和利率。
- 这些模块应按功能独立拆分，避免和 Funding 模块互相污染。

## 低优先级交易扩展

- SpreadTrading：spread instruments、order、cancel、amend、orders、fills。
- BlockTrading / RFQ：counterparties、rfq、quote、execute quote、mmp。
- GridTrading：网格策略创建、停止、查询、历史和收益。
- CopyTrading：带单、跟单、配置、收益和历史。
- FDBroker：broker rebate、用户映射、佣金和关系查询。
- TradingData：成交量、持仓量、多空比、主动买卖、杠杆借贷比等数据接口。
- Status：系统状态、维护窗口和公告类接口。
