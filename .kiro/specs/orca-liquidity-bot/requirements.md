# Requirements Document

## Introduction

Orcaプールでの流動性供給ポジション（SOL-USDC）の収益性を自動的に管理し、最適化するbotシステム。ポジションのレンジ監視、Yieldの自動回収、ポジションの再設定、およびLINE通知機能を提供する。

## Glossary

- **Orca_Pool**: SolanaブロックチェーンのDEXプラットフォームOrcaの流動性プール
- **Position**: 流動性プールに供給された資産のポジション
- **Range**: 流動性供給の価格レンジ
- **Yield**: 流動性供給により得られる収益
- **Bot_System**: 自動化されたポジション管理システム
- **LINE_Bot**: LINE Messaging APIを使用した通知システム
- **Price_Monitor**: 価格監視システム
- **Position_Manager**: ポジション管理システム

## Requirements

### Requirement 1: ポジションレンジ監視

**User Story:** As a liquidity provider, I want to monitor my position range automatically, so that I can maintain optimal yield generation.

#### Acceptance Criteria

1. WHEN the system runs hourly checks, THE Price_Monitor SHALL check if the current position is within the set range
2. WHEN both current price and 1-hour-ago price are outside the range, THE Position_Manager SHALL automatically collect yield and reposition
3. WHEN repositioning occurs, THE Bot_System SHALL calculate new optimal range based on current market conditions
4. WHEN repositioning is completed, THE LINE_Bot SHALL send notification with position details

### Requirement 2: 自動Yield回収

**User Story:** As a liquidity provider, I want to collect yields automatically on a daily basis, so that I can compound my returns efficiently.

#### Acceptance Criteria

1. WHEN the system time reaches 00:00 daily, THE Position_Manager SHALL automatically collect accumulated yield
2. WHEN yield collection is completed, THE Position_Manager SHALL reposition the liquidity with collected yield included
3. WHEN daily yield collection occurs, THE LINE_Bot SHALL send notification with yield and position information

### Requirement 3: LINE通知システム

**User Story:** As a user, I want to receive notifications about my position status, so that I can stay informed about my investment performance.

#### Acceptance Criteria

1. WHEN repositioning occurs due to range deviation, THE LINE_Bot SHALL send notification containing new position range, total balance, SOL/USDC ratio, collected yield amount, and SOL price in USDC
2. WHEN daily yield collection occurs, THE LINE_Bot SHALL send notification containing total balance, SOL/USDC ratio, collected yield amount, and SOL price in USDC
3. WHEN notifications are sent, THE LINE_Bot SHALL format messages in Japanese for user readability
4. WHEN notification sending fails, THE Bot_System SHALL log the error and retry up to 3 times

### Requirement 4: 価格データ取得

**User Story:** As a system, I want to access accurate price data, so that I can make informed positioning decisions.

#### Acceptance Criteria

1. WHEN price monitoring occurs, THE Price_Monitor SHALL fetch current SOL/USDC price from reliable sources
2. WHEN historical price is needed, THE Price_Monitor SHALL retrieve price data from 1 hour ago
3. WHEN price data is unavailable, THE Bot_System SHALL handle the error gracefully and retry
4. WHEN price data is fetched, THE Bot_System SHALL validate data integrity before using it

### Requirement 5: ポジション管理

**User Story:** As a system, I want to manage liquidity positions efficiently, so that I can optimize yield generation.

#### Acceptance Criteria

1. WHEN creating new positions, THE Position_Manager SHALL calculate optimal range based on current volatility and market conditions
2. WHEN collecting yield, THE Position_Manager SHALL execute the collection transaction and confirm completion
3. WHEN repositioning, THE Position_Manager SHALL close existing position and create new position atomically
4. WHEN position operations fail, THE Position_Manager SHALL handle errors and notify via LINE_Bot

### Requirement 6: システム運用

**User Story:** As a system administrator, I want the bot to run reliably in the cloud, so that it can operate continuously without manual intervention.

#### Acceptance Criteria

1. WHEN deployed to Google Cloud Run, THE Bot_System SHALL run scheduled tasks using Cloud Scheduler
2. WHEN system errors occur, THE Bot_System SHALL log detailed error information for debugging
3. WHEN the system starts, THE Bot_System SHALL validate all required configurations and API keys
4. WHEN system resources are low, THE Bot_System SHALL handle resource constraints gracefully