use orca_liquidity_bot::{PriceRange, BotState};
use chrono::Utc;

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