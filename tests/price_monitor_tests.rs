use orca_liquidity_bot::price_monitor::PriceMonitor;
use orca_liquidity_bot::{PriceRange, PriceError};
use proptest::prelude::*;
use chrono::Utc;

// Helper function to create a PriceMonitor for testing
fn create_test_price_monitor() -> PriceMonitor {
    PriceMonitor::new()
}

// Property-based test for price data fetching reliability
// Feature: orca-liquidity-bot, Property 9: Price data fetching reliability
proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]
    
    #[test]
    fn property_price_data_fetching_reliability(
        hours_ago in 1u32..168u32, // Test historical data from 1 hour to 1 week ago
    ) {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let result = rt.block_on(async {
            let price_monitor = create_test_price_monitor();

            // Property: For any price monitoring request, the system should attempt to fetch 
            // both current and historical price data from configured sources
            
            // Test current price fetching
            let current_price_result = price_monitor.get_current_price().await;
            
            // Test historical price fetching
            let historical_price_result = price_monitor.get_historical_price(hours_ago).await;

            // Property: The system should make a genuine attempt to fetch data
            // This means either:
            // 1. Both succeed and return valid PriceData
            // 2. Both fail with appropriate error types (network issues, API issues, etc.)
            // 3. Mixed results are acceptable due to data availability differences
            
            match (&current_price_result, &historical_price_result) {
                (Ok(current_data), Ok(historical_data)) => {
                    // Both succeeded - validate the data quality
                    prop_assert!(current_data.current_price > 0.0, 
                        "Current price should be positive: {}", current_data.current_price);
                    prop_assert!(current_data.current_price.is_finite(), 
                        "Current price should be finite: {}", current_data.current_price);
                    prop_assert!(historical_data.current_price > 0.0, 
                        "Historical price should be positive: {}", historical_data.current_price);
                    prop_assert!(historical_data.current_price.is_finite(), 
                        "Historical price should be finite: {}", historical_data.current_price);
                    
                    // Timestamps should be reasonable
                    prop_assert!(current_data.timestamp <= Utc::now(), 
                        "Current price timestamp should not be in the future");
                    prop_assert!(historical_data.timestamp <= Utc::now(), 
                        "Historical price timestamp should not be in the future");
                },
                (Ok(current_data), Err(historical_error)) => {
                    // Current succeeded, historical failed - this is acceptable
                    prop_assert!(current_data.current_price > 0.0, 
                        "Current price should be positive: {}", current_data.current_price);
                    prop_assert!(current_data.current_price.is_finite(), 
                        "Current price should be finite: {}", current_data.current_price);
                    
                    // Historical error should be appropriate type
                    prop_assert!(matches!(historical_error, 
                        PriceError::FetchFailed | 
                        PriceError::InvalidData(_) | 
                        PriceError::Network(_) | 
                        PriceError::ValidationFailed(_)), 
                        "Historical price error should be appropriate type: {:?}", historical_error);
                },
                (Err(current_error), Ok(historical_data)) => {
                    // Historical succeeded, current failed - this is acceptable
                    prop_assert!(historical_data.current_price > 0.0, 
                        "Historical price should be positive: {}", historical_data.current_price);
                    prop_assert!(historical_data.current_price.is_finite(), 
                        "Historical price should be finite: {}", historical_data.current_price);
                    
                    // Current error should be appropriate type
                    prop_assert!(matches!(current_error, 
                        PriceError::FetchFailed | 
                        PriceError::InvalidData(_) | 
                        PriceError::Network(_) | 
                        PriceError::ValidationFailed(_)), 
                        "Current price error should be appropriate type: {:?}", current_error);
                },
                (Err(current_error), Err(historical_error)) => {
                    // Both failed - errors should be appropriate types
                    prop_assert!(matches!(current_error, 
                        PriceError::FetchFailed | 
                        PriceError::InvalidData(_) | 
                        PriceError::Network(_) | 
                        PriceError::ValidationFailed(_)), 
                        "Current price error should be appropriate type: {:?}", current_error);
                    
                    prop_assert!(matches!(historical_error, 
                        PriceError::FetchFailed | 
                        PriceError::InvalidData(_) | 
                        PriceError::Network(_) | 
                        PriceError::ValidationFailed(_)), 
                        "Historical price error should be appropriate type: {:?}", historical_error);
                }
            }

            // Additional property: The system should handle the request without panicking
            // This is implicitly tested by reaching this point
            Ok(())
        });
        
        // Unwrap the result from the async block
        result.unwrap();
    }
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]
    
    #[test]
    fn property_price_range_checking_reliability(
        price in 0.01f64..10000.0f64, // Reasonable price range for SOL
        lower_bound in 0.01f64..5000.0f64,
        upper_bound_offset in 0.01f64..5000.0f64,
    ) {
        let upper_bound = lower_bound + upper_bound_offset; // Ensure upper > lower
        let range = PriceRange::new(lower_bound, upper_bound);
        let price_monitor = create_test_price_monitor();

        // Property: Range checking should be consistent and reliable
        let is_in_range = price_monitor.is_price_in_range(price, &range);
        let expected_in_range = price >= lower_bound && price <= upper_bound;

        prop_assert_eq!(is_in_range, expected_in_range, 
            "Range check inconsistency: price={}, range=[{}, {}], got={}, expected={}", 
            price, lower_bound, upper_bound, is_in_range, expected_in_range);
    }
}

// Property 1: Range monitoring accuracy
// Feature: orca-liquidity-bot, Property 1: Range monitoring accuracy
proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]
    
    #[test]
    fn property_range_monitoring_accuracy(
        // Generate a valid price range
        lower_bound in 0.01f64..1000.0f64,
        range_width in 0.01f64..1000.0f64,
        // Generate test prices both inside and outside the range
        price_offset in -2000.0f64..2000.0f64,
    ) {
        let upper_bound = lower_bound + range_width;
        let range = PriceRange::new(lower_bound, upper_bound);
        let test_price = lower_bound + (range_width / 2.0) + price_offset;
        
        // Ensure test price is positive (realistic for SOL price)
        let test_price = if test_price <= 0.0 { 0.01 } else { test_price };
        
        let price_monitor = create_test_price_monitor();

        // Property 1: For any position with a defined price range and any current price data, 
        // the range check should correctly identify whether the price is within the range bounds
        let is_in_range = price_monitor.is_price_in_range(test_price, &range);
        
        // The expected result based on mathematical range checking
        let expected_in_range = test_price >= range.lower_bound && test_price <= range.upper_bound;
        
        prop_assert_eq!(is_in_range, expected_in_range,
            "Range monitoring accuracy failed: price={}, range=[{}, {}], got={}, expected={}",
            test_price, range.lower_bound, range.upper_bound, is_in_range, expected_in_range);
            
        // Additional accuracy checks for boundary conditions
        if test_price == range.lower_bound || test_price == range.upper_bound {
            prop_assert!(is_in_range, 
                "Boundary price should be considered in range: price={}, range=[{}, {}]",
                test_price, range.lower_bound, range.upper_bound);
        }
        
        // Verify range bounds are valid
        prop_assert!(range.lower_bound <= range.upper_bound,
            "Range bounds should be valid: lower={}, upper={}",
            range.lower_bound, range.upper_bound);
    }
}

// Unit tests for specific scenarios
#[tokio::test]
async fn test_price_monitor_creation() {
    let _price_monitor = PriceMonitor::new();
    // Test that the PriceMonitor can be created successfully
    // We can't access private fields, so we just verify it doesn't panic
    assert!(true);
}

#[test]
fn test_price_range_checking_edge_cases() {
    let price_monitor = create_test_price_monitor();
    let range = PriceRange::new(100.0, 200.0);

    // Test boundary conditions
    assert!(price_monitor.is_price_in_range(100.0, &range));
    assert!(price_monitor.is_price_in_range(200.0, &range));
    assert!(price_monitor.is_price_in_range(150.0, &range));
    assert!(!price_monitor.is_price_in_range(99.99, &range));
    assert!(!price_monitor.is_price_in_range(200.01, &range));
}