# Orca Liquidity Bot

Orcaプールでの流動性供給ポジション（SOL-USDC）を自動管理するRustベースのbotシステム。

## Project Structure

```
src/
├── lib.rs                 # Library entry point
├── main.rs               # Application entry point
├── types.rs              # Core data types and structures
├── errors.rs             # Error type definitions
├── price_monitor/        # Price monitoring component
├── position_manager/     # Position management component
├── line_notifier/        # LINE notification component
├── state_manager/        # State persistence component
└── scheduler/            # Main scheduler handler
```

## Core Types

- `PriceRange`: Price range for liquidity positions
- `Position`: Liquidity position information
- `PriceData`: Price data with timestamp
- `BotState`: Bot state for persistence
- `NotificationData`: Data for LINE notifications
- `BotConfig`: System configuration

## Error Types

- `BotError`: Main error type encompassing all errors
- `PriceError`: Price monitoring related errors
- `PositionError`: Position management related errors
- `NotificationError`: LINE notification related errors
- `StateError`: State management related errors

## Environment Variables

- `LINE_CHANNEL_ACCESS_TOKEN`: LINE Bot channel access token
- `LINE_USER_ID`: LINE user ID for notifications
- `WALLET_PRIVATE_KEY`: Solana wallet private key
- `RPC_ENDPOINT`: Solana RPC endpoint (optional, defaults to mainnet)
- `STATE_FILE_PATH`: Path for state persistence (optional, defaults to /tmp/bot_state.json)

## Development

```bash
# Check code
cargo check

# Run tests
cargo test

# Build
cargo build --release
```

## Original Requirements

### Positionのレンジ監視
→1時間おきにセットしたPositionがレンジの範囲内かどうかを監視し、現在価格と1時間前の価格がいずれもレンジから逸脱している場合はYieldの回収とPositionの引き直しを自動で行う
→引き直し後、

1. 新しいPositionのレンジ
2. Positionの総残高
3. SOL/USDCの比率
4. 回収したYieldの量
5. SOLのUSDC建ての価格

をLINE botに通知する

### Yieldの回収
→1日に一回0時にYieldを回収し、Positionを引き直す
→上記のタイミングで

1. Positionの総残高
2. SOL/USDCの比率
3. 回収したYieldの量
4. SOLのUSDC建ての価格

をLINE botに通知する

## 技術選定

### 言語
Rust

### デプロイ先
Google Cloud(Cloud Run)

## Next Steps

This is the initial project structure. Individual components will be implemented in subsequent tasks according to the implementation plan.