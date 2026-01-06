use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Price range for liquidity positions
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PriceRange {
    pub lower_bound: f64,
    pub upper_bound: f64,
}

impl PriceRange {
    pub fn new(lower_bound: f64, upper_bound: f64) -> Self {
        Self {
            lower_bound,
            upper_bound,
        }
    }

    pub fn contains(&self, price: f64) -> bool {
        price >= self.lower_bound && price <= self.upper_bound
    }
}

/// Liquidity position information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub id: String,
    pub range: PriceRange,
    pub sol_amount: f64,
    pub usdc_amount: f64,
    pub total_value_usdc: f64,
    pub created_at: DateTime<Utc>,
}

/// Price data with timestamp
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceData {
    pub current_price: f64,
    pub timestamp: DateTime<Utc>,
}

/// Bot state for persistence
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BotState {
    pub last_check_time: DateTime<Utc>,
    pub current_position: Option<Position>,
    pub last_yield_collection: DateTime<Utc>,
}

impl Default for BotState {
    fn default() -> Self {
        let now = Utc::now();
        Self {
            last_check_time: now,
            current_position: None,
            last_yield_collection: now,
        }
    }
}

/// Notification data for LINE messages
#[derive(Debug, Clone)]
pub struct NotificationData {
    pub position_range: Option<PriceRange>,
    pub total_balance: f64,
    pub sol_usdc_ratio: (f64, f64),
    pub collected_yield: f64,
    pub sol_price_usdc: f64,
}

/// Configuration for the bot system
#[derive(Debug, Clone)]
pub struct BotConfig {
    pub line_channel_access_token: String,
    pub line_user_id: String,
    pub wallet_private_key: String,
    pub rpc_endpoint: String,
    pub state_file_path: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_price_range_contains() {
        let range = PriceRange::new(100.0, 200.0);
        
        assert!(range.contains(150.0));
        assert!(range.contains(100.0));
        assert!(range.contains(200.0));
        assert!(!range.contains(50.0));
        assert!(!range.contains(250.0));
    }

    #[test]
    fn test_price_range_serialization() {
        let range = PriceRange::new(100.0, 200.0);
        let json = serde_json::to_string(&range).unwrap();
        let deserialized: PriceRange = serde_json::from_str(&json).unwrap();
        
        assert_eq!(range, deserialized);
    }

    #[test]
    fn test_bot_state_default() {
        let state = BotState::default();
        
        assert!(state.current_position.is_none());
        assert!(state.last_check_time <= Utc::now());
        assert!(state.last_yield_collection <= Utc::now());
    }
}