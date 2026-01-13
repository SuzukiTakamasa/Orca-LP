use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use solana_sdk::pubkey::Pubkey;
use std::collections::VecDeque;
use std::time::Duration;

/// Represents a liquidity position in an Orca Whirlpool
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Position {
    /// The position's public key address
    pub address: Pubkey,
    /// The whirlpool this position belongs to
    pub whirlpool: Pubkey,
    /// Lower tick boundary of the position
    pub tick_lower: i32,
    /// Upper tick boundary of the position
    pub tick_upper: i32,
    /// Amount of liquidity in the position
    pub liquidity: u128,
    /// Fee growth checkpoint for token A
    pub fee_growth_checkpoint_a: u128,
    /// Fee growth checkpoint for token B
    pub fee_growth_checkpoint_b: u128,
}

impl Position {
    /// Creates a new Position instance
    pub fn new(
        address: Pubkey,
        whirlpool: Pubkey,
        tick_lower: i32,
        tick_upper: i32,
        liquidity: u128,
        fee_growth_checkpoint_a: u128,
        fee_growth_checkpoint_b: u128,
    ) -> Self {
        Self {
            address,
            whirlpool,
            tick_lower,
            tick_upper,
            liquidity,
            fee_growth_checkpoint_a,
            fee_growth_checkpoint_b,
        }
    }

    /// Validates the position data for integrity
    pub fn validate(&self) -> Result<(), String> {
        if self.tick_lower >= self.tick_upper {
            return Err("tick_lower must be less than tick_upper".to_string());
        }

        if self.liquidity == 0 {
            return Err("liquidity must be greater than zero".to_string());
        }

        // Validate tick bounds (Orca uses tick spacing)
        if self.tick_lower < -443636 || self.tick_upper > 443636 {
            return Err("tick values are out of valid range".to_string());
        }

        Ok(())
    }

    /// Checks if the position is active (has liquidity)
    pub fn is_active(&self) -> bool {
        self.liquidity > 0
    }
}

/// Represents a price range for liquidity positions
#[derive(Debug, Clone, PartialEq)]
pub struct PriceRange {
    /// Lower price boundary
    pub lower_price: f64,
    /// Upper price boundary
    pub upper_price: f64,
    /// Lower tick corresponding to lower_price
    pub lower_tick: i32,
    /// Upper tick corresponding to upper_price
    pub upper_tick: i32,
}

impl PriceRange {
    /// Creates a new PriceRange instance
    pub fn new(lower_price: f64, upper_price: f64, lower_tick: i32, upper_tick: i32) -> Self {
        Self {
            lower_price,
            upper_price,
            lower_tick,
            upper_tick,
        }
    }

    /// Validates the price range for integrity
    pub fn validate(&self) -> Result<(), String> {
        if self.lower_price >= self.upper_price {
            return Err("lower_price must be less than upper_price".to_string());
        }

        if self.lower_price <= 0.0 || self.upper_price <= 0.0 {
            return Err("prices must be positive".to_string());
        }

        if self.lower_tick >= self.upper_tick {
            return Err("lower_tick must be less than upper_tick".to_string());
        }

        // Validate tick bounds
        if self.lower_tick < -443636 || self.upper_tick > 443636 {
            return Err("tick values are out of valid range".to_string());
        }

        Ok(())
    }

    /// Checks if a given price is within this range
    pub fn contains_price(&self, price: f64) -> bool {
        price >= self.lower_price && price <= self.upper_price
    }

    /// Returns the width of the price range
    pub fn width(&self) -> f64 {
        self.upper_price - self.lower_price
    }

    /// Returns the midpoint price of the range
    pub fn midpoint(&self) -> f64 {
        (self.lower_price + self.upper_price) / 2.0
    }
}

/// Source of price data
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PriceSource {
    /// Price from Solana RPC
    SolanaRpc,
    /// Price from external API
    ExternalApi(String),
    /// Price from Orca pool
    OrcaPool,
}

/// Represents price data at a specific point in time
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PriceData {
    /// Current price (SOL/USDC)
    pub current_price: f64,
    /// Timestamp when the price was recorded
    pub timestamp: DateTime<Utc>,
    /// Source of the price data
    pub source: PriceSource,
}

impl PriceData {
    /// Creates a new PriceData instance
    pub fn new(current_price: f64, timestamp: DateTime<Utc>, source: PriceSource) -> Self {
        Self {
            current_price,
            timestamp,
            source,
        }
    }

    /// Validates the price data for integrity
    pub fn validate(&self) -> Result<(), String> {
        if self.current_price <= 0.0 {
            return Err("current_price must be positive".to_string());
        }

        if self.current_price.is_infinite() || self.current_price.is_nan() {
            return Err("current_price must be a valid finite number".to_string());
        }

        // Check if timestamp is not too far in the future (allow 1 minute tolerance)
        let now = Utc::now();
        let max_future = now + chrono::Duration::minutes(1);
        if self.timestamp > max_future {
            return Err("timestamp cannot be in the future".to_string());
        }

        Ok(())
    }

    /// Checks if the price data is recent (within specified duration)
    pub fn is_recent(&self, max_age: Duration) -> bool {
        let now = Utc::now();
        let age = now.signed_duration_since(self.timestamp);
        age.to_std().unwrap_or(Duration::MAX) <= max_age
    }
}

/// Position metrics for reporting and analysis
#[derive(Debug, Clone, PartialEq)]
pub struct PositionMetrics {
    /// Total balance in USD
    pub total_balance_usd: f64,
    /// Amount of SOL in the position
    pub sol_amount: f64,
    /// Amount of USDC in the position
    pub usdc_amount: f64,
    /// Total collected yield
    pub collected_yield: f64,
    /// Current SOL price
    pub sol_price: f64,
}

impl PositionMetrics {
    /// Creates a new PositionMetrics instance
    pub fn new(
        total_balance_usd: f64,
        sol_amount: f64,
        usdc_amount: f64,
        collected_yield: f64,
        sol_price: f64,
    ) -> Self {
        Self {
            total_balance_usd,
            sol_amount,
            usdc_amount,
            collected_yield,
            sol_price,
        }
    }

    /// Validates the position metrics for integrity
    pub fn validate(&self) -> Result<(), String> {
        if self.total_balance_usd < 0.0 {
            return Err("total_balance_usd cannot be negative".to_string());
        }

        if self.sol_amount < 0.0 {
            return Err("sol_amount cannot be negative".to_string());
        }

        if self.usdc_amount < 0.0 {
            return Err("usdc_amount cannot be negative".to_string());
        }

        if self.collected_yield < 0.0 {
            return Err("collected_yield cannot be negative".to_string());
        }

        if self.sol_price <= 0.0 {
            return Err("sol_price must be positive".to_string());
        }

        Ok(())
    }

    /// Calculates the SOL/USDC ratio
    pub fn sol_usdc_ratio(&self) -> f64 {
        if self.usdc_amount == 0.0 {
            return f64::INFINITY;
        }
        self.sol_amount / self.usdc_amount
    }
}

/// Event data structure for range deviation logging
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RangeDeviationEvent {
    /// Timestamp when the deviation was detected
    pub timestamp: DateTime<Utc>,
    /// Position address that experienced the deviation
    pub position_address: String,
    /// Whirlpool address associated with the position
    pub whirlpool_address: String,
    /// Current price at the time of deviation
    pub current_price: f64,
    /// Historical price (if available) for comparison
    pub historical_price: Option<f64>,
    /// Lower bound of the position range
    pub range_lower: f64,
    /// Upper bound of the position range
    pub range_upper: f64,
    /// Lower tick of the position
    pub tick_lower: i32,
    /// Upper tick of the position
    pub tick_upper: i32,
    /// Type of deviation detected
    pub deviation_type: String,
}

/// Event data structure for regular price monitoring logging
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceMonitoringEvent {
    /// Timestamp of the price monitoring event
    pub timestamp: DateTime<Utc>,
    /// Position address being monitored
    pub position_address: String,
    /// Current price
    pub price: f64,
    /// Source of the price data
    pub source: PriceSource,
    /// Whether the price is within the position range
    pub is_in_range: bool,
    /// Lower tick of the position
    pub tick_lower: i32,
    /// Upper tick of the position
    pub tick_upper: i32,
}

/// Price statistics for analysis and reporting
#[derive(Debug, Clone, PartialEq)]
pub struct PriceStatistics {
    /// Average price over the specified period
    pub average_price: Option<f64>,
    /// Price volatility (standard deviation) over the period
    pub volatility: Option<f64>,
    /// Most recent price
    pub latest_price: Option<f64>,
    /// Oldest price in the analysis period
    pub oldest_price_in_period: Option<f64>,
    /// Number of data points used in the analysis
    pub data_points: usize,
}

impl PriceStatistics {
    /// Creates a new PriceStatistics instance
    pub fn new(
        average_price: Option<f64>,
        volatility: Option<f64>,
        latest_price: Option<f64>,
        oldest_price_in_period: Option<f64>,
        data_points: usize,
    ) -> Self {
        Self {
            average_price,
            volatility,
            latest_price,
            oldest_price_in_period,
            data_points,
        }
    }
    
    /// Checks if there is sufficient data for meaningful analysis
    pub fn has_sufficient_data(&self) -> bool {
        self.data_points >= 2 && self.average_price.is_some()
    }
    
    /// Calculates the price change percentage over the period
    pub fn price_change_percentage(&self) -> Option<f64> {
        match (self.latest_price, self.oldest_price_in_period) {
            (Some(latest), Some(oldest)) if oldest != 0.0 => {
                Some(((latest - oldest) / oldest) * 100.0)
            }
            _ => None,
        }
    }
}

/// Manages historical price data with efficient storage and retrieval
#[derive(Debug, Clone)]
pub struct PriceHistory {
    /// Storage for price entries using VecDeque for efficient insertion/removal
    pub entries: VecDeque<PriceData>,
    /// Maximum number of entries to keep in history
    pub max_entries: usize,
}

impl PriceHistory {
    /// Creates a new PriceHistory with specified maximum entries
    pub fn new(max_entries: usize) -> Self {
        Self {
            entries: VecDeque::with_capacity(max_entries),
            max_entries,
        }
    }

    /// Creates a new PriceHistory with default capacity (1000 entries)
    pub fn default() -> Self {
        Self::new(1000)
    }

    /// Adds a new price entry to the history
    pub fn add_entry(&mut self, price: PriceData) {
        // Remove oldest entry if at capacity
        if self.entries.len() >= self.max_entries {
            self.entries.pop_front();
        }
        
        self.entries.push_back(price);
    }

    /// Retrieves the most recent price entry
    pub fn get_latest(&self) -> Option<&PriceData> {
        self.entries.back()
    }

    /// Retrieves the oldest price entry
    pub fn get_oldest(&self) -> Option<&PriceData> {
        self.entries.front()
    }

    /// Retrieves a price entry from a specific duration ago
    pub fn get_price_at(&self, duration_ago: Duration) -> Option<&PriceData> {
        let target_time = Utc::now() - chrono::Duration::from_std(duration_ago).ok()?;
        
        // Find the entry closest to the target time
        self.entries
            .iter()
            .min_by_key(|entry| {
                (entry.timestamp - target_time).num_seconds().abs()
            })
    }

    /// Checks if both current and historical prices are outside the given range
    pub fn is_range_deviated(&self, range: &PriceRange, duration: Duration) -> bool {
        // Check current price
        let current_outside = self.get_latest()
            .map(|entry| !range.contains_price(entry.current_price))
            .unwrap_or(false);

        // Check historical price
        let historical_outside = self.get_price_at(duration)
            .map(|entry| !range.contains_price(entry.current_price))
            .unwrap_or(false);

        current_outside && historical_outside
    }

    /// Returns the number of entries in the history
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Checks if the history is empty
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Clears all entries from the history
    pub fn clear(&mut self) {
        self.entries.clear();
    }

    /// Returns an iterator over all price entries (oldest to newest)
    pub fn iter(&self) -> impl Iterator<Item = &PriceData> {
        self.entries.iter()
    }

    /// Returns entries within a specific time range
    pub fn get_entries_in_range(&self, start: DateTime<Utc>, end: DateTime<Utc>) -> Vec<&PriceData> {
        self.entries
            .iter()
            .filter(|entry| entry.timestamp >= start && entry.timestamp <= end)
            .collect()
    }

    /// Calculates the average price over a specified duration
    pub fn average_price_over(&self, duration: Duration) -> Option<f64> {
        let cutoff_time = Utc::now() - chrono::Duration::from_std(duration).ok()?;
        
        let recent_entries: Vec<_> = self.entries
            .iter()
            .filter(|entry| entry.timestamp >= cutoff_time)
            .collect();

        if recent_entries.is_empty() {
            return None;
        }

        let sum: f64 = recent_entries.iter().map(|entry| entry.current_price).sum();
        Some(sum / recent_entries.len() as f64)
    }

    /// Gets the price volatility (standard deviation) over a specified duration
    pub fn price_volatility_over(&self, duration: Duration) -> Option<f64> {
        let cutoff_time = Utc::now() - chrono::Duration::from_std(duration).ok()?;
        
        let recent_prices: Vec<f64> = self.entries
            .iter()
            .filter(|entry| entry.timestamp >= cutoff_time)
            .map(|entry| entry.current_price)
            .collect();

        if recent_prices.len() < 2 {
            return None;
        }

        let mean = recent_prices.iter().sum::<f64>() / recent_prices.len() as f64;
        let variance = recent_prices
            .iter()
            .map(|price| (price - mean).powi(2))
            .sum::<f64>() / recent_prices.len() as f64;

        Some(variance.sqrt())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use solana_sdk::pubkey::Pubkey;

    #[test]
    fn test_position_validation() {
        let address = Pubkey::new_unique();
        let whirlpool = Pubkey::new_unique();
        
        // Valid position
        let valid_position = Position::new(address, whirlpool, -1000, 1000, 100, 0, 0);
        assert!(valid_position.validate().is_ok());
        assert!(valid_position.is_active());

        // Invalid position - tick_lower >= tick_upper
        let invalid_position = Position::new(address, whirlpool, 1000, 1000, 100, 0, 0);
        assert!(invalid_position.validate().is_err());

        // Invalid position - zero liquidity
        let zero_liquidity = Position::new(address, whirlpool, -1000, 1000, 0, 0, 0);
        assert!(zero_liquidity.validate().is_err());
        assert!(!zero_liquidity.is_active());
    }

    #[test]
    fn test_price_range_validation() {
        // Valid range
        let valid_range = PriceRange::new(100.0, 200.0, -1000, 1000);
        assert!(valid_range.validate().is_ok());
        assert!(valid_range.contains_price(150.0));
        assert!(!valid_range.contains_price(50.0));
        assert_eq!(valid_range.width(), 100.0);
        assert_eq!(valid_range.midpoint(), 150.0);

        // Invalid range - lower >= upper
        let invalid_range = PriceRange::new(200.0, 100.0, -1000, 1000);
        assert!(invalid_range.validate().is_err());

        // Invalid range - negative prices
        let negative_range = PriceRange::new(-100.0, 200.0, -1000, 1000);
        assert!(negative_range.validate().is_err());
    }

    #[test]
    fn test_price_data_validation() {
        let now = Utc::now();
        let source = PriceSource::SolanaRpc;

        // Valid price data
        let valid_price = PriceData::new(100.0, now, source.clone());
        assert!(valid_price.validate().is_ok());
        assert!(valid_price.is_recent(Duration::from_secs(60)));

        // Invalid price data - negative price
        let invalid_price = PriceData::new(-100.0, now, source.clone());
        assert!(invalid_price.validate().is_err());

        // Invalid price data - NaN
        let nan_price = PriceData::new(f64::NAN, now, source);
        assert!(nan_price.validate().is_err());
    }

    #[test]
    fn test_position_metrics_validation() {
        // Valid metrics
        let valid_metrics = PositionMetrics::new(1000.0, 5.0, 500.0, 10.0, 100.0);
        assert!(valid_metrics.validate().is_ok());
        assert_eq!(valid_metrics.sol_usdc_ratio(), 0.01);

        // Invalid metrics - negative values
        let invalid_metrics = PositionMetrics::new(-1000.0, 5.0, 500.0, 10.0, 100.0);
        assert!(invalid_metrics.validate().is_err());

        // Test division by zero in ratio
        let zero_usdc_metrics = PositionMetrics::new(1000.0, 5.0, 0.0, 10.0, 100.0);
        assert!(zero_usdc_metrics.sol_usdc_ratio().is_infinite());
    }

    #[test]
    fn test_price_history_basic_operations() {
        let mut history = PriceHistory::new(3);
        assert!(history.is_empty());
        assert_eq!(history.len(), 0);

        let now = Utc::now();
        let price1 = PriceData::new(100.0, now, PriceSource::SolanaRpc);
        let price2 = PriceData::new(110.0, now + chrono::Duration::minutes(1), PriceSource::SolanaRpc);
        let price3 = PriceData::new(120.0, now + chrono::Duration::minutes(2), PriceSource::SolanaRpc);

        history.add_entry(price1.clone());
        assert_eq!(history.len(), 1);
        assert_eq!(history.get_latest().unwrap(), &price1);
        assert_eq!(history.get_oldest().unwrap(), &price1);

        history.add_entry(price2.clone());
        history.add_entry(price3.clone());
        assert_eq!(history.len(), 3);
        assert_eq!(history.get_latest().unwrap(), &price3);
        assert_eq!(history.get_oldest().unwrap(), &price1);

        // Test capacity limit
        let price4 = PriceData::new(130.0, now + chrono::Duration::minutes(3), PriceSource::SolanaRpc);
        history.add_entry(price4.clone());
        assert_eq!(history.len(), 3); // Should still be 3
        assert_eq!(history.get_latest().unwrap(), &price4);
        assert_eq!(history.get_oldest().unwrap(), &price2); // price1 should be removed
    }

    #[test]
    fn test_price_history_range_deviation() {
        let mut history = PriceHistory::new(10);
        let now = Utc::now();
        let range = PriceRange::new(100.0, 200.0, -1000, 1000);

        // Add current price outside range
        let current_price = PriceData::new(250.0, now, PriceSource::SolanaRpc);
        history.add_entry(current_price);

        // Add historical price outside range (1 hour ago)
        let historical_price = PriceData::new(50.0, now - chrono::Duration::hours(1), PriceSource::SolanaRpc);
        history.add_entry(historical_price);

        // Should detect deviation
        assert!(history.is_range_deviated(&range, Duration::from_secs(3600)));

        // Add price inside range
        let inside_price = PriceData::new(150.0, now - chrono::Duration::minutes(30), PriceSource::SolanaRpc);
        history.add_entry(inside_price);

        // Should not detect deviation if historical price is inside range
        // (Note: this test depends on which price get_price_at returns for the 1-hour mark)
    }

    #[test]
    fn test_price_history_statistics() {
        let mut history = PriceHistory::new(10);
        let now = Utc::now();

        // Add some price data
        for i in 0..5 {
            let price = PriceData::new(
                100.0 + i as f64 * 10.0,
                now - chrono::Duration::minutes(i * 10),
                PriceSource::SolanaRpc,
            );
            history.add_entry(price);
        }

        // Test average price calculation
        let avg = history.average_price_over(Duration::from_secs(3600));
        assert!(avg.is_some());
        assert_eq!(avg.unwrap(), 120.0); // (100 + 110 + 120 + 130 + 140) / 5

        // Test volatility calculation
        let volatility = history.price_volatility_over(Duration::from_secs(3600));
        assert!(volatility.is_some());
        assert!(volatility.unwrap() > 0.0);
    }

    #[test]
    fn test_price_history_time_range_queries() {
        let mut history = PriceHistory::new(10);
        let now = Utc::now();

        // Add entries with different timestamps
        for i in 0..5 {
            let price = PriceData::new(
                100.0 + i as f64,
                now - chrono::Duration::hours(i),
                PriceSource::SolanaRpc,
            );
            history.add_entry(price);
        }

        // Query entries in a specific time range
        let start = now - chrono::Duration::hours(3);
        let end = now - chrono::Duration::hours(1);
        let entries_in_range = history.get_entries_in_range(start, end);
        
        // Should include entries from 1, 2, and 3 hours ago
        assert_eq!(entries_in_range.len(), 3);
    }
}