# Requirements Document

## Introduction

OrcaプールでのSOL-USDC流動性供給ポジションの収益性を自動管理し、レンジ逸脱時の自動調整とYield回収を行うbotシステム。価格監視、自動リバランス、LINE通知機能を提供する。

## Glossary

- **Orca_Bot**: SOL-USDC流動性供給ポジションを自動管理するメインシステム
- **Position**: Orcaプールでの流動性供給ポジション
- **Range**: 流動性供給の価格レンジ（上限・下限価格）
- **Yield**: 流動性供給により獲得した手数料収益
- **Price_Monitor**: 価格監視コンポーネント
- **LINE_Notifier**: LINE Bot通知システム
- **Rebalancer**: ポジション再調整コンポーネント

## Requirements

### Requirement 1: Position Range Monitoring

**User Story:** As a liquidity provider, I want to monitor my position range automatically, so that I can ensure my position remains active and profitable.

#### Acceptance Criteria

1. WHEN the system runs hourly checks, THE Price_Monitor SHALL retrieve current SOL/USDC price and compare it to the position range
2. WHEN both current price and 1-hour-ago price are outside the position range, THE Orca_Bot SHALL trigger automatic yield collection and position rebalancing
3. WHEN price monitoring occurs, THE Price_Monitor SHALL store price history for comparison
4. WHEN range deviation is detected, THE Orca_Bot SHALL log the deviation event with timestamp and price data

### Requirement 2: Automatic Yield Collection and Rebalancing

**User Story:** As a liquidity provider, I want automatic yield collection and position rebalancing, so that I can maintain optimal returns without manual intervention.

#### Acceptance Criteria

1. WHEN range deviation triggers rebalancing, THE Rebalancer SHALL collect all accumulated yield from the current position
2. WHEN yield is collected, THE Rebalancer SHALL close the existing position and create a new position with updated range
3. WHEN rebalancing is complete, THE Orca_Bot SHALL calculate new position parameters (total balance, SOL/USDC ratio, collected yield amount, current SOL price)
4. WHEN new position is established, THE LINE_Notifier SHALL send notification with position details

### Requirement 3: Daily Yield Collection

**User Story:** As a liquidity provider, I want daily yield collection at midnight, so that I can compound my returns regularly.

#### Acceptance Criteria

1. WHEN the system clock reaches 00:00 JST daily, THE Orca_Bot SHALL trigger yield collection process
2. WHEN daily yield collection occurs, THE Rebalancer SHALL collect accumulated yield and reestablish the position
3. WHEN daily collection is complete, THE Orca_Bot SHALL calculate position metrics (total balance, SOL/USDC ratio, yield amount, SOL price)
4. WHEN daily metrics are calculated, THE LINE_Notifier SHALL send daily report notification

### Requirement 4: LINE Bot Notifications

**User Story:** As a liquidity provider, I want to receive LINE notifications about my position status, so that I can stay informed about my investment performance.

#### Acceptance Criteria

1. WHEN position rebalancing occurs, THE LINE_Notifier SHALL send a message containing new position total balance, SOL/USDC ratio, collected yield amount, and current SOL/USDC price
2. WHEN daily yield collection occurs, THE LINE_Notifier SHALL send a daily report with the same metrics
3. WHEN notification sending fails, THE LINE_Notifier SHALL retry up to 3 times with exponential backoff
4. WHEN all retry attempts fail, THE Orca_Bot SHALL log the notification failure for manual review

### Requirement 5: Orca SDK Integration

**User Story:** As a system operator, I want seamless integration with Orca protocol, so that the bot can interact with Whirlpool positions reliably.

#### Acceptance Criteria

1. WHEN connecting to Orca, THE Orca_Bot SHALL use the official Orca SDK for all pool interactions
2. WHEN retrieving position data, THE Orca_Bot SHALL query current position status, range, and accumulated fees
3. WHEN creating new positions, THE Orca_Bot SHALL calculate optimal range based on current market conditions and volatility
4. WHEN interacting with Solana blockchain, THE Orca_Bot SHALL handle transaction failures gracefully with appropriate retry logic

### Requirement 6: Cloud Infrastructure Integration

**User Story:** As a system operator, I want the bot to run reliably on Google Cloud infrastructure, so that it operates continuously without manual intervention.

#### Acceptance Criteria

1. WHEN deploying the system, THE Orca_Bot SHALL run as a containerized service on Google Cloud Run
2. WHEN scheduling periodic tasks, THE system SHALL use Google Cloud Scheduler for hourly monitoring and daily yield collection
3. WHEN managing infrastructure, THE system SHALL be provisioned and managed through Terraform
4. WHEN system errors occur, THE Orca_Bot SHALL log errors to Google Cloud Logging for monitoring and debugging

### Requirement 7: Error Handling and Recovery

**User Story:** As a system operator, I want robust error handling, so that temporary failures don't disrupt the automated operations.

#### Acceptance Criteria

1. WHEN Solana RPC calls fail, THE Orca_Bot SHALL retry with exponential backoff up to 5 times
2. WHEN Orca SDK operations fail, THE Orca_Bot SHALL log the error and attempt recovery procedures
3. WHEN critical errors occur that prevent operation, THE Orca_Bot SHALL send emergency notifications via LINE
4. WHEN system recovers from errors, THE Orca_Bot SHALL resume normal operations and log recovery status

### Requirement 8: Configuration Management

**User Story:** As a system operator, I want configurable parameters, so that I can adjust bot behavior without code changes.

#### Acceptance Criteria

1. WHEN the system starts, THE Orca_Bot SHALL load configuration from environment variables or config files
2. WHEN configuration includes sensitive data, THE Orca_Bot SHALL retrieve secrets from Google Secret Manager
3. WHEN configuration parameters change, THE Orca_Bot SHALL support hot reloading without service restart
4. WHEN invalid configuration is detected, THE Orca_Bot SHALL fail fast with clear error messages