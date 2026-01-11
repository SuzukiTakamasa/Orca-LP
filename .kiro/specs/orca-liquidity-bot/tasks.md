# Implementation Plan: Orca Liquidity Bot

## Overview

Rustを使用してOrcaプロトコル上でSOL-USDC流動性供給ポジションを自動管理するbotを実装します。Google Cloud上で動作し、価格監視、自動リバランス、LINE通知機能を提供します。

## Tasks

- [x] 1. Set up project structure and dependencies
  - Create Rust project with Cargo.toml
  - Add required dependencies: tokio, serde, reqwest, solana-client, orca-whirlpools-sdk, thiserror, chrono, proptest
  - Set up basic project structure with modules
  - Configure logging with env_logger
  - _Requirements: 8.1_

- [ ] 2. Implement configuration management
  - [ ] 2.1 Create Config struct and environment variable loading
    - Define Config struct with all required fields
    - Implement environment variable loading with validation
    - _Requirements: 8.1, 8.4_

  - [ ]* 2.2 Write property test for configuration loading
    - **Property 13: Configuration loading robustness**
    - **Validates: Requirements 8.1**

  - [ ] 2.3 Implement Google Secret Manager integration
    - Add secret retrieval functionality for sensitive configuration
    - Ensure secrets are never logged or exposed
    - _Requirements: 8.2_

  - [ ]* 2.4 Write property test for secret management
    - **Property 14: Secret management security**
    - **Validates: Requirements 8.2**

  - [ ] 2.5 Add configuration validation and hot reload support
    - Implement configuration validation with clear error messages
    - Add hot reload functionality for configuration changes
    - _Requirements: 8.3, 8.4_

  - [ ]* 2.6 Write property tests for configuration validation and hot reload
    - **Property 15: Hot reload functionality**
    - **Property 16: Configuration validation**
    - **Validates: Requirements 8.3, 8.4**

- [ ] 3. Implement core data models and error types
  - [ ] 3.1 Create Position, PriceRange, and PriceData structs
    - Define all data structures with proper serialization
    - Implement validation methods for data integrity
    - _Requirements: 1.1, 1.3, 2.1_

  - [ ] 3.2 Implement PriceHistory with storage and retrieval
    - Create PriceHistory struct with VecDeque storage
    - Add methods for storing and retrieving historical price data
    - _Requirements: 1.3_

  - [ ]* 3.3 Write property test for price history persistence
    - **Property 3: Price history persistence**
    - **Validates: Requirements 1.3**

  - [ ] 3.4 Define comprehensive error types and BotError enum
    - Create error types for all components with proper error chaining
    - Implement Display and Error traits for all error types
    - _Requirements: 7.1, 7.2, 7.3_

- [ ] 4. Implement Price Monitor component
  - [ ] 4.1 Create PriceMonitor struct with RPC client integration
    - Implement Solana RPC client setup and price retrieval
    - Add price comparison logic for range checking
    - _Requirements: 1.1, 1.2_

  - [ ]* 4.2 Write property tests for price monitoring
    - **Property 1: Price monitoring and comparison**
    - **Property 2: Range deviation trigger logic**
    - **Validates: Requirements 1.1, 1.2**

  - [ ] 4.3 Add price history management and deviation detection
    - Implement price storage and historical comparison
    - Add logic to detect when both current and 1-hour-ago prices are outside range
    - _Requirements: 1.2, 1.3_

  - [ ] 4.4 Implement event logging for range deviations
    - Add structured logging for deviation events with timestamps and price data
    - _Requirements: 1.4_

  - [ ]* 4.5 Write property test for event logging
    - **Property 4: Event logging completeness**
    - **Validates: Requirements 1.4**

- [ ] 5. Checkpoint - Ensure price monitoring tests pass
  - Ensure all tests pass, ask the user if questions arise.

- [ ] 6. Implement Rebalancer component
  - [ ] 6.1 Create Rebalancer struct with Orca SDK integration
    - Set up Orca Whirlpool client and wallet integration
    - Implement position data retrieval from Orca protocol
    - _Requirements: 5.1, 5.2_

  - [ ]* 6.2 Write property test for position data retrieval
    - **Property 10: Position data retrieval completeness**
    - **Validates: Requirements 5.2**

  - [ ] 6.3 Implement yield collection and position management
    - Add yield collection from existing positions
    - Implement position closing and creation logic
    - _Requirements: 2.1, 2.2_

  - [ ] 6.4 Add optimal range calculation based on market conditions
    - Implement range calculation using volatility and market data
    - Ensure calculated ranges have valid tick values
    - _Requirements: 5.3_

  - [ ]* 6.5 Write property tests for rebalancing operations
    - **Property 5: Yield collection and position lifecycle**
    - **Property 11: Range calculation validity**
    - **Validates: Requirements 2.1, 2.2, 5.3**

  - [ ] 6.6 Implement position metrics calculation
    - Calculate total balance, SOL/USDC ratio, collected yield, and current prices
    - Ensure calculations accurately reflect position data
    - _Requirements: 2.3, 3.3_

  - [ ]* 6.7 Write property test for metrics calculation
    - **Property 6: Position metrics calculation accuracy**
    - **Validates: Requirements 2.3, 3.3**

- [ ] 7. Implement LINE Notifier component
  - [ ] 7.1 Create LineNotifier struct with LINE Messaging API integration
    - Set up LINE Bot client with channel access token
    - Implement message formatting for different notification types
    - _Requirements: 2.4, 3.4, 4.1, 4.2_

  - [ ]* 7.2 Write property test for notification content
    - **Property 7: Notification content completeness**
    - **Validates: Requirements 2.4, 3.4, 4.1, 4.2**

  - [ ] 7.3 Add retry logic with exponential backoff for notifications
    - Implement retry mechanism for failed notification attempts
    - Add exponential backoff timing between retries
    - _Requirements: 4.3_

  - [ ] 7.4 Implement emergency notification functionality
    - Add emergency alert sending for critical errors
    - Ensure emergency notifications bypass normal retry limits
    - _Requirements: 7.3_

  - [ ]* 7.5 Write property tests for notification reliability
    - **Property 12: Emergency notification reliability**
    - **Validates: Requirements 7.3**

- [ ] 8. Implement error handling and recovery system
  - [ ] 8.1 Add comprehensive error handling with retry logic
    - Implement retry mechanisms for transient errors (RPC, network)
    - Add exponential backoff for Solana RPC calls and Orca SDK operations
    - _Requirements: 7.1, 7.2, 5.4_

  - [ ]* 8.2 Write property tests for retry mechanisms
    - **Property 8: Retry logic with exponential backoff**
    - **Validates: Requirements 4.3, 5.4, 7.1**

  - [ ] 8.3 Implement error logging and recovery procedures
    - Add structured logging for all error types and recovery attempts
    - Implement recovery status logging when system recovers from errors
    - _Requirements: 6.4, 7.2, 7.4_

  - [ ]* 8.4 Write property test for error logging consistency
    - **Property 9: Error logging consistency**
    - **Validates: Requirements 4.4, 6.4, 7.2, 7.4**

- [ ] 9. Implement main OrcaBot orchestration
  - [ ] 9.1 Create OrcaBot struct integrating all components
    - Wire together PriceMonitor, Rebalancer, LineNotifier, and Config
    - Implement main workflow orchestration methods
    - _Requirements: 1.1, 1.2, 2.1, 2.2_

  - [ ] 9.2 Add hourly monitoring workflow
    - Implement run_hourly_check method with price monitoring and deviation detection
    - Integrate automatic rebalancing trigger when range deviation is detected
    - _Requirements: 1.1, 1.2, 2.1, 2.2_

  - [ ] 9.3 Add daily yield collection workflow
    - Implement run_daily_collection method for scheduled yield collection
    - Ensure daily collection follows same rebalancing logic as range-triggered collection
    - _Requirements: 3.1, 3.2, 3.3, 3.4_

  - [ ] 9.4 Implement emergency error handling
    - Add handle_emergency method for critical error scenarios
    - Ensure emergency notifications are sent for system-threatening errors
    - _Requirements: 7.3, 7.4_

- [ ] 10. Add Google Cloud integration
  - [ ] 10.1 Create Cloud Run compatible main function
    - Implement HTTP server for Cloud Run deployment
    - Add health check endpoints and graceful shutdown
    - _Requirements: 6.1_

  - [ ] 10.2 Add Cloud Scheduler integration
    - Create HTTP endpoints for hourly and daily scheduled tasks
    - Implement request authentication for scheduled calls
    - _Requirements: 6.2_

  - [ ] 10.3 Integrate Google Cloud Logging
    - Configure structured logging to Google Cloud Logging
    - Ensure all error and operational logs are properly formatted
    - _Requirements: 6.4_

- [ ] 11. Create deployment infrastructure
  - [ ] 11.1 Create Dockerfile for containerization
    - Write multi-stage Dockerfile for optimized Rust builds
    - Configure proper security settings and non-root user
    - _Requirements: 6.1_

  - [ ] 11.2 Create Terraform configuration for Google Cloud resources
    - Define Cloud Run service, Cloud Scheduler jobs, and Secret Manager
    - Configure IAM roles and permissions for secure operation
    - _Requirements: 6.3_

  - [ ]* 11.3 Write integration tests for deployment
    - Test containerized application startup and configuration loading
    - Verify Cloud Scheduler integration and HTTP endpoint functionality
    - _Requirements: 6.1, 6.2_

- [ ] 12. Final integration and testing
  - [ ] 12.1 Write comprehensive integration tests with mocked services
    - Test end-to-end workflows with mocked external services (LINE API, price feeds)
    - Verify error handling and recovery across component boundaries
    - _Requirements: All requirements_

  - [ ] 12.2 Implement E2E tests using Solana devnet
    - Set up devnet test environment with test SOL and USDC tokens
    - Create actual Orca Whirlpool positions on devnet for realistic testing
    - Test complete workflows: position creation, yield collection, rebalancing with real blockchain interactions
    - Verify LINE notifications are sent with actual position data from devnet
    - Test price monitoring and automatic rebalancing triggers using devnet price feeds
    - _Requirements: 1.1, 1.2, 2.1, 2.2, 2.3, 2.4, 3.2, 3.3, 3.4_

  - [ ]* 12.3 Add performance and load testing
    - Test high-frequency price monitoring performance
    - Verify system stability under sustained operation
    - _Requirements: 1.1, 1.3_

  - [ ] 12.4 Create deployment and operation documentation
    - Document deployment procedures and configuration requirements
    - Add troubleshooting guide and monitoring recommendations
    - Include devnet testing procedures and setup instructions
    - _Requirements: 8.1, 8.2_

- [ ] 13. Final checkpoint - Ensure all tests pass and system is ready for deployment
  - Ensure all tests pass, ask the user if questions arise.

## Notes

- Tasks marked with `*` are optional and can be skipped for faster MVP
- Each task references specific requirements for traceability
- Property tests validate universal correctness properties with minimum 100 iterations
- Unit tests validate specific examples and edge cases
- Integration tests ensure components work together correctly with mocked dependencies
- E2E tests use Solana devnet for realistic blockchain interactions and actual position management
- All sensitive operations (mainnet transactions) should only occur in production environment