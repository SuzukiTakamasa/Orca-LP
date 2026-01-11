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
Currently, there are complex version conflicts between the Orca Whirlpools SDK and the Solana ecosystem dependencies. The project structure is fully prepared for Orca SDK integration:

- `Rebalancer` struct has placeholder for `WhirlpoolsClient`
- All necessary imports and method signatures are ready
- The dependency is commented out in `Cargo.toml` with detailed notes

**Version Conflict Details:**
- `orca_whirlpools` v6.0 requires newer cryptographic dependencies (`zeroize` v1.5+)
- `solana-sdk` v1.16-1.18 uses older cryptographic dependencies (`zeroize` <1.4)
- This creates an irreconcilable dependency conflict

**Resolution Options:**
1. Wait for Solana ecosystem to update to newer cryptographic libraries
2. Use a fork of orca_whirlpools with compatible dependencies
3. Implement Orca protocol interactions directly using Anchor/Solana primitives

The bot's core functionality (price monitoring, notifications, scheduling) is fully implemented and ready to use.

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