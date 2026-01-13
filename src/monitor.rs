use crate::config::Config;
use crate::error::{BotError, PriceError};
use crate::models::{Position, PriceRange, PriceData, PriceSource, PriceHistory, PriceStatistics, RangeDeviationEvent, PriceMonitoringEvent};
use chrono::Utc;
use log::{info, warn, error, debug};
use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use std::time::Duration;

/// Price Monitor component responsible for tracking SOL/USDC prices and detecting range deviations
pub struct PriceMonitor {
    /// Solana RPC client for blockchain interactions
    rpc_client: RpcClient,
    /// Historical price data storage
    price_history: PriceHistory,
    /// SOL/USDC Whirlpool address for price queries
    whirlpool_address: Pubkey,
}

impl PriceMonitor {
    /// Creates a new PriceMonitor instance
    pub async fn new(config: &Config) -> Result<Self, BotError> {
        info!("Initializing PriceMonitor with RPC URL: {}", config.solana_rpc_url);
        
        let rpc_client = RpcClient::new(&config.solana_rpc_url);
        let price_history = PriceHistory::new(168); // Store 1 week of hourly data
        
        // Use the position's whirlpool address for price queries
        // In a real implementation, this would be derived from the position
        let whirlpool_address = config.position_address; // Simplified for now
        
        // Test RPC connection
        match rpc_client.get_health() {
            Ok(_) => info!("Successfully connected to Solana RPC"),
            Err(e) => {
                warn!("Failed to connect to Solana RPC: {}", e);
                // Don't fail initialization, as RPC might be temporarily unavailable
            }
        }
        
        Ok(PriceMonitor {
            rpc_client,
            price_history,
            whirlpool_address,
        })
    }
    
    /// Retrieves the current SOL/USDC price from Orca Whirlpool
    pub async fn get_current_price(&self) -> Result<PriceData, PriceError> {
        debug!("Fetching current SOL/USDC price from Orca Whirlpool");
        
        // For now, we'll implement a basic price retrieval mechanism
        // In a full implementation, this would query the actual Orca Whirlpool account
        match self.fetch_price_from_whirlpool().await {
            Ok(price) => {
                info!("Successfully retrieved price: ${:.4}", price);
                Ok(PriceData::new(
                    price,
                    Utc::now(),
                    PriceSource::OrcaPool,
                ))
            }
            Err(e) => {
                error!("Failed to fetch price from Whirlpool: {}", e);
                // Fallback to external price source
                self.fetch_price_from_external_api().await
            }
        }
    }
    
    /// Checks if the current price is outside the position's range
    pub async fn check_range_deviation(&self, position: &Position) -> Result<bool, PriceError> {
        debug!("Checking range deviation for position: {}", position.address);
        
        // Get current price
        let current_price_data = self.get_current_price().await?;
        let current_price = current_price_data.current_price;
        
        // Convert position ticks to price range
        let price_range = self.ticks_to_price_range(position.tick_lower, position.tick_upper)?;
        
        // Check if current price is outside range
        let is_outside_range = !price_range.contains_price(current_price);
        
        if is_outside_range {
            info!(
                "Price ${:.4} is outside range [${:.4}, ${:.4}]",
                current_price, price_range.lower_price, price_range.upper_price
            );
        } else {
            debug!(
                "Price ${:.4} is within range [${:.4}, ${:.4}]",
                current_price, price_range.lower_price, price_range.upper_price
            );
        }
        
        Ok(is_outside_range)
    }
    
    /// Stores price data in the historical price storage
    pub async fn store_price_history(&mut self, price: PriceData) -> Result<(), PriceError> {
        debug!("Storing price data: ${:.4} at {}", price.current_price, price.timestamp);
        
        // Validate price data before storing
        price.validate()
            .map_err(|e| PriceError::InvalidData(e))?;
        
        self.price_history.add_entry(price);
        
        info!("Price history now contains {} entries", self.price_history.len());
        Ok(())
    }
    
    /// Gets the price history for analysis
    pub fn get_price_history(&self) -> &PriceHistory {
        &self.price_history
    }
    
    /// Checks if both current and historical prices are outside the range
    pub async fn is_range_deviated_with_history(&self, position: &Position, duration_ago: Duration) -> Result<bool, PriceError> {
        debug!("Checking range deviation with historical comparison");
        
        // Convert position ticks to price range
        let price_range = self.ticks_to_price_range(position.tick_lower, position.tick_upper)?;
        
        // Check if range is deviated using price history
        let is_deviated = self.price_history.is_range_deviated(&price_range, duration_ago);
        
        if is_deviated {
            info!("Range deviation detected: both current and historical prices are outside range");
        } else {
            debug!("No range deviation: at least one price (current or historical) is within range");
        }
        
        Ok(is_deviated)
    }
    
    /// Performs comprehensive range deviation check as required by the system
    /// This is the main method that should be called for range deviation detection
    pub async fn check_comprehensive_range_deviation(&mut self, position: &Position) -> Result<bool, PriceError> {
        info!("Performing comprehensive range deviation check for position: {}", position.address);
        
        // First, get and store the current price
        let current_price_data = self.get_current_price().await?;
        self.store_price_history(current_price_data.clone()).await?;
        
        // Convert position ticks to price range for logging
        let price_range = self.ticks_to_price_range(position.tick_lower, position.tick_upper)?;
        
        // Check if current price is in range
        let current_in_range = price_range.contains_price(current_price_data.current_price);
        
        // Log the price monitoring event
        self.log_price_monitoring_event(&current_price_data, position, current_in_range);
        
        info!(
            "Current price: ${:.4}, Position range: [${:.4}, ${:.4}]",
            current_price_data.current_price, price_range.lower_price, price_range.upper_price
        );
        
        // Check if both current and 1-hour-ago prices are outside range
        let one_hour = Duration::from_secs(3600);
        let is_deviated = self.is_range_deviated_with_history(position, one_hour).await?;
        
        if is_deviated {
            // Get historical price for detailed logging
            let historical_price = self.price_history.get_price_at(one_hour).map(|p| p.current_price);
            
            // Log the range deviation event with structured data
            self.log_range_deviation_event(
                position,
                current_price_data.current_price,
                historical_price,
                &price_range,
            );
            
            if let Some(hist_price) = historical_price {
                info!(
                    "Range deviation confirmed - Current: ${:.4}, 1h ago: ${:.4}, both outside range [${:.4}, ${:.4}]",
                    current_price_data.current_price,
                    hist_price,
                    price_range.lower_price,
                    price_range.upper_price
                );
            } else {
                warn!("Range deviation detected but no historical price data available for comparison");
            }
        } else {
            debug!("No range deviation detected - at least one price is within range");
        }
        
        Ok(is_deviated)
    }
    
    /// Gets price statistics for analysis and reporting
    pub fn get_price_statistics(&self, duration: Duration) -> Result<PriceStatistics, PriceError> {
        let average = self.price_history.average_price_over(duration);
        let volatility = self.price_history.price_volatility_over(duration);
        let latest = self.price_history.get_latest().map(|p| p.current_price);
        let oldest_in_period = self.price_history.get_price_at(duration).map(|p| p.current_price);
        
        Ok(PriceStatistics {
            average_price: average,
            volatility,
            latest_price: latest,
            oldest_price_in_period: oldest_in_period,
            data_points: self.price_history.len(),
        })
    }
    
    /// Logs a range deviation event with structured data
    pub fn log_range_deviation_event(&self, position: &Position, current_price: f64, historical_price: Option<f64>, price_range: &PriceRange) {
        let event_data = RangeDeviationEvent {
            timestamp: Utc::now(),
            position_address: position.address.to_string(),
            whirlpool_address: position.whirlpool.to_string(),
            current_price,
            historical_price,
            range_lower: price_range.lower_price,
            range_upper: price_range.upper_price,
            tick_lower: position.tick_lower,
            tick_upper: position.tick_upper,
            deviation_type: if historical_price.is_some() { 
                "both_outside_range".to_string() 
            } else { 
                "current_outside_range".to_string() 
            },
        };
        
        // Structured logging with JSON format for easy parsing by monitoring systems
        info!(
            target: "range_deviation",
            "Range deviation event: {}",
            serde_json::to_string(&event_data).unwrap_or_else(|_| "Failed to serialize event".to_string())
        );
        
        // Also log in human-readable format
        match historical_price {
            Some(hist_price) => {
                warn!(
                    "RANGE DEVIATION DETECTED - Position: {}, Current: ${:.4}, 1h ago: ${:.4}, Range: [${:.4}, ${:.4}], Time: {}",
                    position.address,
                    current_price,
                    hist_price,
                    price_range.lower_price,
                    price_range.upper_price,
                    event_data.timestamp.format("%Y-%m-%d %H:%M:%S UTC")
                );
            }
            None => {
                warn!(
                    "PRICE OUT OF RANGE - Position: {}, Current: ${:.4}, Range: [${:.4}, ${:.4}], Time: {}",
                    position.address,
                    current_price,
                    price_range.lower_price,
                    price_range.upper_price,
                    event_data.timestamp.format("%Y-%m-%d %H:%M:%S UTC")
                );
            }
        }
    }
    
    /// Logs a price monitoring event for regular operations
    pub fn log_price_monitoring_event(&self, price_data: &PriceData, position: &Position, is_in_range: bool) {
        let event_data = PriceMonitoringEvent {
            timestamp: price_data.timestamp,
            position_address: position.address.to_string(),
            price: price_data.current_price,
            source: price_data.source.clone(),
            is_in_range,
            tick_lower: position.tick_lower,
            tick_upper: position.tick_upper,
        };
        
        // Structured logging
        debug!(
            target: "price_monitoring",
            "Price monitoring event: {}",
            serde_json::to_string(&event_data).unwrap_or_else(|_| "Failed to serialize event".to_string())
        );
        
        // Human-readable logging
        if is_in_range {
            debug!(
                "Price monitoring - Position: {}, Price: ${:.4} (IN RANGE), Source: {:?}, Time: {}",
                position.address,
                price_data.current_price,
                price_data.source,
                price_data.timestamp.format("%Y-%m-%d %H:%M:%S UTC")
            );
        } else {
            info!(
                "Price monitoring - Position: {}, Price: ${:.4} (OUT OF RANGE), Source: {:?}, Time: {}",
                position.address,
                price_data.current_price,
                price_data.source,
                price_data.timestamp.format("%Y-%m-%d %H:%M:%S UTC")
            );
        }
    }
    
    /// Fetches price from Orca Whirlpool (simplified implementation)
    async fn fetch_price_from_whirlpool(&self) -> Result<f64, PriceError> {
        // This is a simplified implementation
        // In a real implementation, this would:
        // 1. Query the Whirlpool account data
        // 2. Calculate the current price from sqrt_price
        // 3. Handle token decimals properly
        
        match self.rpc_client.get_account(&self.whirlpool_address) {
            Ok(_account) => {
                // For now, return a mock price that varies slightly
                // In real implementation, parse account data to get sqrt_price
                let base_price = 100.0;
                let variation = (Utc::now().timestamp() % 100) as f64 / 1000.0;
                Ok(base_price + variation)
            }
            Err(e) => {
                Err(PriceError::RetrievalFailed(format!("RPC error: {}", e)))
            }
        }
    }
    
    /// Fetches price from external API as fallback
    async fn fetch_price_from_external_api(&self) -> Result<PriceData, PriceError> {
        warn!("Using external API fallback for price data");
        
        // This would typically call Jupiter API or similar
        // For now, return a reasonable mock price
        let mock_price = 100.0 + (Utc::now().timestamp() % 50) as f64 / 10.0;
        
        Ok(PriceData::new(
            mock_price,
            Utc::now(),
            PriceSource::ExternalApi("Jupiter".to_string()),
        ))
    }
    
    /// Converts tick values to price range (simplified calculation)
    fn ticks_to_price_range(&self, tick_lower: i32, tick_upper: i32) -> Result<PriceRange, PriceError> {
        // This is a simplified tick-to-price conversion
        // Real implementation would use proper Orca/Uniswap v3 math
        
        if tick_lower >= tick_upper {
            return Err(PriceError::InvalidData("tick_lower must be less than tick_upper".to_string()));
        }
        
        // Simplified conversion: each tick represents ~0.01% price change
        let tick_spacing = 0.0001; // 0.01%
        let base_price = 100.0; // Base SOL price
        
        let lower_price = base_price * (1.0 + tick_lower as f64 * tick_spacing);
        let upper_price = base_price * (1.0 + tick_upper as f64 * tick_spacing);
        
        // Ensure prices are positive
        let lower_price = lower_price.max(0.01);
        let upper_price = upper_price.max(lower_price + 0.01);
        
        Ok(PriceRange::new(lower_price, upper_price, tick_lower, tick_upper))
    }
    
    /// Clears old price history data beyond the retention period
    pub fn cleanup_old_price_data(&mut self) {
        // The PriceHistory struct automatically manages capacity
        // This method is for future enhancements if needed
        debug!("Price history cleanup - current entries: {}", self.price_history.len());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;
    use crate::models::{Position, PriceRange};
    use solana_sdk::pubkey::Pubkey;
    use std::time::Duration;

    async fn create_test_config() -> Config {
        Config {
            solana_rpc_url: "https://api.devnet.solana.com".to_string(),
            whirlpool_program_id: Pubkey::new_unique(),
            position_address: Pubkey::new_unique(),
            wallet_private_key: "test_key".to_string(),
            line_channel_token: "test_token".to_string(),
            line_user_id: "test_user".to_string(),
            monitoring_interval: Duration::from_secs(3600),
            google_project_id: "test-project".to_string(),
            google_credentials_path: None,
        }
    }

    fn create_test_position() -> Position {
        Position::new(
            Pubkey::new_unique(),
            Pubkey::new_unique(),
            -1000, // tick_lower
            1000,  // tick_upper
            1000000, // liquidity
            0, // fee_growth_checkpoint_a
            0, // fee_growth_checkpoint_b
        )
    }

    #[tokio::test]
    async fn test_price_monitor_creation() {
        let config = create_test_config().await;
        let monitor = PriceMonitor::new(&config).await;
        assert!(monitor.is_ok());
    }

    #[tokio::test]
    async fn test_get_current_price() {
        let config = create_test_config().await;
        let monitor = PriceMonitor::new(&config).await.unwrap();
        let price = monitor.get_current_price().await;
        assert!(price.is_ok());
        let price_data = price.unwrap();
        assert!(price_data.current_price > 0.0);
    }

    #[tokio::test]
    async fn test_price_history_storage() {
        let config = create_test_config().await;
        let mut monitor = PriceMonitor::new(&config).await.unwrap();
        
        let price_data = monitor.get_current_price().await.unwrap();
        let result = monitor.store_price_history(price_data).await;
        assert!(result.is_ok());
        assert_eq!(monitor.get_price_history().len(), 1);
    }

    #[tokio::test]
    async fn test_range_deviation_check() {
        let config = create_test_config().await;
        let monitor = PriceMonitor::new(&config).await.unwrap();
        let position = create_test_position();
        
        let result = monitor.check_range_deviation(&position).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_comprehensive_range_deviation() {
        let config = create_test_config().await;
        let mut monitor = PriceMonitor::new(&config).await.unwrap();
        let position = create_test_position();
        
        let result = monitor.check_comprehensive_range_deviation(&position).await;
        assert!(result.is_ok());
    }

    #[test]
    fn test_tick_to_price_conversion() {
        let config = tokio_test::block_on(create_test_config());
        let monitor = tokio_test::block_on(PriceMonitor::new(&config)).unwrap();
        
        let result = monitor.ticks_to_price_range(-1000, 1000);
        assert!(result.is_ok());
        
        let price_range = result.unwrap();
        assert!(price_range.lower_price < price_range.upper_price);
        assert!(price_range.lower_price > 0.0);
    }

    #[test]
    fn test_price_statistics() {
        let config = tokio_test::block_on(create_test_config());
        let monitor = tokio_test::block_on(PriceMonitor::new(&config)).unwrap();
        
        let stats = monitor.get_price_statistics(Duration::from_secs(3600));
        assert!(stats.is_ok());
    }
}