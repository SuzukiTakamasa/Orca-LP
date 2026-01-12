# Orca Liquidity Bot

Automated SOL-USDC liquidity position management bot for Orca protocol on Solana.

## Features

- Automated price monitoring and range deviation detection
- Automatic yield collection and position rebalancing
- LINE Bot notifications for position updates
- Google Cloud deployment ready
- Comprehensive error handling and recovery

## Setup

### Prerequisites

- Rust 1.70+ 
- Solana CLI tools
- Google Cloud SDK (for deployment)

### Known Issues

**Orca Whirlpools SDK Integration**: 
Currently, there are complex version conflicts between the Orca Whirlpools SDK and the Solana ecosystem dependencies. However, **Orca operations can be performed without the SDK** using direct Solana/Anchor program calls.

**Direct Implementation Approach:**
The bot is designed to interact directly with the Orca Whirlpool program using:
- **Program ID**: `whirLbMiicVdio4qvUfM5KAg6Ct8VwpYzGff3uctyCc`
- **Anchor Client**: For building and sending transactions
- **SPL Token**: For token operations and transfers

**Key Operations Available:**
1. **Yield Collection**: Direct `collect_fees` instruction calls
2. **Position Management**: `open_position`, `close_position`, `increase_liquidity`, `decrease_liquidity`
3. **Price Monitoring**: Reading Whirlpool account data for current prices
4. **Range Calculations**: Computing optimal tick ranges based on market conditions

**Advantages of Direct Implementation:**
- No SDK version conflicts
- Full control over transaction construction
- Lower-level access to Whirlpool program features
- Reduced dependency footprint

**Implementation Status:**
- Project structure is ready for direct Whirlpool program interaction
- Core instruction builders are outlined in `src/rebalancer.rs`
- All necessary Solana/Anchor dependencies are included

### Installation

1. Clone the repository
2. Copy `.env.example` to `.env` and configure your settings:

```bash
cp .env.example .env
```

3. Install dependencies:

```bash
cargo build
```

### Configuration

Set the following environment variables in your `.env` file:

- `SOLANA_RPC_URL`: Solana RPC endpoint
- `WHIRLPOOL_PROGRAM_ID`: Orca Whirlpool program ID
- `POSITION_ADDRESS`: Your liquidity position address
- `WALLET_PRIVATE_KEY`: Your wallet private key (keep secure!)
- `LINE_CHANNEL_TOKEN`: LINE Bot channel access token
- `LINE_USER_ID`: Your LINE user ID for notifications
- `MONITORING_INTERVAL`: Price monitoring interval in seconds (default: 3600)

### Running

```bash
# Development
RUST_LOG=info cargo run

# Production build
cargo build --release
```

### Testing

```bash
# Run all tests
cargo test

# Run property-based tests
cargo test --features proptest
```

## Architecture

The bot consists of several key components:

- **Price Monitor**: Monitors SOL/USDC price and detects range deviations
- **Rebalancer**: Handles yield collection and position rebalancing
- **LINE Notifier**: Sends notifications via LINE Bot
- **Configuration Manager**: Manages settings and secrets

## Deployment

The bot is designed to run on Google Cloud Run with Cloud Scheduler for periodic execution.

See the deployment documentation for detailed setup instructions.

## License

MIT License