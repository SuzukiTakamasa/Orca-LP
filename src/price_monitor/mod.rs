use crate::{PriceData, PriceError, PriceRange};
use chrono::{DateTime, Duration, Utc};
use reqwest::Client;
use serde::Deserialize;
use std::time::Duration as StdDuration;
use tokio::time::sleep;
use tracing::{debug, error, warn};

/// CoinGecko API response for price data
#[derive(Debug, Deserialize)]
struct CoinGeckoResponse {
    solana: CoinGeckoPrice,
}

#[derive(Debug, Deserialize)]
struct CoinGeckoPrice {
    usd: f64,
}

/// CoinGecko historical price response
#[derive(Debug, Deserialize)]
struct CoinGeckoHistoricalResponse {
    prices: Vec<[f64; 2]>, // [timestamp_ms, price]
}

/// Price monitoring component
pub struct PriceMonitor {
    client: Client,
    coingecko_base_url: String,
    max_retries: u32,
    base_retry_delay: StdDuration,
}

impl PriceMonitor {
    pub fn new() -> Self {
        let client = Client::builder()
            .timeout(StdDuration::from_secs(30))
            .user_agent("orca-liquidity-bot/0.1.0")
            .build()
            .expect("Failed to create HTTP client");

        Self {
            client,
            coingecko_base_url: "https://api.coingecko.com/api/v3".to_string(),
            max_retries: 3,
            base_retry_delay: StdDuration::from_millis(1000),
        }
    }

    /// Execute a request with retry logic and exponential backoff
    /// Implements graceful error handling with detailed logging for debugging
    async fn execute_with_retry<F, Fut, T>(&self, operation: F) -> Result<T, PriceError>
    where
        F: Fn() -> Fut,
        Fut: std::future::Future<Output = Result<T, PriceError>>,
    {
        let mut last_error = None;
        
        for attempt in 0..=self.max_retries {
            match operation().await {
                Ok(result) => {
                    if attempt > 0 {
                        debug!("Price data fetch succeeded on attempt {}", attempt + 1);
                    }
                    return Ok(result);
                },
                Err(error) => {
                    // Log detailed error information for debugging
                    match &error {
                        PriceError::Network(e) => {
                            warn!(
                                "Network error on attempt {} of {}: {}",
                                attempt + 1,
                                self.max_retries + 1,
                                e
                            );
                        },
                        PriceError::FetchFailed => {
                            warn!(
                                "API fetch failed on attempt {} of {}: Server returned error status",
                                attempt + 1,
                                self.max_retries + 1
                            );
                        },
                        PriceError::InvalidData(msg) => {
                            warn!(
                                "Invalid data on attempt {} of {}: {}",
                                attempt + 1,
                                self.max_retries + 1,
                                msg
                            );
                        },
                        PriceError::ValidationFailed(msg) => {
                            error!(
                                "Data validation failed on attempt {} of {}: {}",
                                attempt + 1,
                                self.max_retries + 1,
                                msg
                            );
                            // Validation failures are likely persistent, but we still retry
                            // in case it was a temporary data corruption issue
                        },
                    }
                    
                    last_error = Some(error);
                    
                    // Don't sleep after the last attempt
                    if attempt < self.max_retries {
                        let delay = self.base_retry_delay * 2_u32.pow(attempt);
                        warn!(
                            "Retrying price data fetch in {:?} (attempt {} of {})",
                            delay,
                            attempt + 2,
                            self.max_retries + 1
                        );
                        sleep(delay).await;
                    }
                }
            }
        }
        
        let final_error = last_error.unwrap();
        error!(
            "All {} price data fetch attempts failed. Final error: {}",
            self.max_retries + 1,
            final_error
        );
        
        // Return the most recent error for upstream handling
        Err(final_error)
    }

    /// Validate price data integrity with comprehensive checks
    /// Implements requirement 4.4: validate data integrity before using it
    fn validate_price_data(&self, price: f64, context: &str) -> Result<(), PriceError> {
        // Check for positive values
        if price <= 0.0 {
            return Err(PriceError::ValidationFailed(format!(
                "{}: Price must be positive, got {}",
                context, price
            )));
        }
        
        // Check for finite values (not NaN or infinite)
        if !price.is_finite() {
            return Err(PriceError::ValidationFailed(format!(
                "{}: Price must be finite, got {}",
                context, price
            )));
        }
        
        // Check for reasonable price bounds (SOL typically trades between $1-$1000)
        // This helps catch obvious data corruption or API errors
        if price < 0.01 || price > 10000.0 {
            return Err(PriceError::ValidationFailed(format!(
                "{}: Price {} is outside reasonable bounds ($0.01 - $10,000)",
                context, price
            )));
        }
        
        // Additional validation: Check for suspiciously precise values that might indicate
        // placeholder or test data (e.g., exactly 100.0, 1.0, etc.)
        if price == 1.0 || price == 100.0 || price == 1000.0 {
            warn!(
                "{}: Price {} looks like a placeholder value, but proceeding",
                context, price
            );
        }
        
        debug!("{}: Price validation passed for ${}", context, price);
        Ok(())
    }

    /// Get current SOL/USDC price from CoinGecko
    /// Implements requirements 4.1, 4.3, 4.4 with retry logic and validation
    pub async fn get_current_price(&self) -> Result<PriceData, PriceError> {
        self.execute_with_retry(|| async {
            let url = format!(
                "{}/simple/price?ids=solana&vs_currencies=usd",
                self.coingecko_base_url
            );

            debug!("Fetching current price from: {}", url);

            let response = self
                .client
                .get(&url)
                .send()
                .await
                .map_err(|e| {
                    warn!("Network error fetching current price: {}", e);
                    // Check for specific network error types for better error handling
                    if e.is_timeout() {
                        warn!("Request timed out - API may be slow or unavailable");
                    } else if e.is_connect() {
                        warn!("Connection failed - API may be down or network issues");
                    } else if e.is_request() {
                        warn!("Request error - malformed request or client issue");
                    }
                    PriceError::Network(e)
                })?;

            // Enhanced status code handling
            let status = response.status();
            if !status.is_success() {
                warn!("API returned non-success status: {} {}", status.as_u16(), status.canonical_reason().unwrap_or("Unknown"));
                
                // Handle specific HTTP status codes
                match status.as_u16() {
                    429 => {
                        warn!("Rate limited by API - will retry with backoff");
                        return Err(PriceError::FetchFailed);
                    },
                    500..=599 => {
                        warn!("Server error - API may be experiencing issues");
                        return Err(PriceError::FetchFailed);
                    },
                    400..=499 => {
                        error!("Client error - may indicate configuration issue");
                        return Err(PriceError::FetchFailed);
                    },
                    _ => {
                        warn!("Unexpected status code: {}", status);
                        return Err(PriceError::FetchFailed);
                    }
                }
            }

            // Parse response with enhanced error handling
            let response_text = response.text().await.map_err(|e| {
                warn!("Failed to read response body: {}", e);
                PriceError::Network(e)
            })?;

            let price_response: CoinGeckoResponse = serde_json::from_str(&response_text)
                .map_err(|e| {
                    warn!("Failed to parse current price JSON: {}", e);
                    warn!("Response body was: {}", response_text);
                    PriceError::InvalidData(format!("Failed to parse JSON: {}", e))
                })?;

            let price = price_response.solana.usd;
            
            // Validate price data integrity (requirement 4.4)
            self.validate_price_data(price, "Current price")?;

            debug!("Successfully fetched current price: ${}", price);

            Ok(PriceData {
                current_price: price,
                timestamp: Utc::now(),
            })
        }).await
    }

    /// Get historical price from specified hours ago
    /// Implements requirements 4.2, 4.3, 4.4 with retry logic and validation
    pub async fn get_historical_price(&self, hours_ago: u32) -> Result<PriceData, PriceError> {
        // Validate input parameters
        if hours_ago == 0 {
            return Err(PriceError::InvalidData(
                "hours_ago must be greater than 0".to_string()
            ));
        }
        
        if hours_ago > 8760 { // More than 1 year
            warn!("Requesting historical data from {} hours ago (more than 1 year)", hours_ago);
        }

        self.execute_with_retry(|| async {
            let target_time = Utc::now() - Duration::hours(hours_ago as i64);
            let from_timestamp = target_time.timestamp();
            let to_timestamp = (target_time + Duration::minutes(5)).timestamp(); // Small window for data availability

            let url = format!(
                "{}/coins/solana/market_chart/range?vs_currency=usd&from={}&to={}",
                self.coingecko_base_url, from_timestamp, to_timestamp
            );

            debug!("Fetching historical price from: {}", url);

            let response = self
                .client
                .get(&url)
                .send()
                .await
                .map_err(|e| {
                    warn!("Network error fetching historical price: {}", e);
                    // Enhanced network error handling
                    if e.is_timeout() {
                        warn!("Historical price request timed out - API may be slow");
                    } else if e.is_connect() {
                        warn!("Connection failed for historical price - API may be down");
                    }
                    PriceError::Network(e)
                })?;

            // Enhanced status code handling for historical data
            let status = response.status();
            if !status.is_success() {
                warn!("API returned non-success status for historical data: {} {}", 
                    status.as_u16(), status.canonical_reason().unwrap_or("Unknown"));
                
                match status.as_u16() {
                    429 => {
                        warn!("Rate limited by API for historical data - will retry");
                        return Err(PriceError::FetchFailed);
                    },
                    404 => {
                        warn!("Historical data not found for {} hours ago", hours_ago);
                        return Err(PriceError::InvalidData(format!(
                            "Historical data not available for {} hours ago", hours_ago
                        )));
                    },
                    500..=599 => {
                        warn!("Server error for historical data - API may be experiencing issues");
                        return Err(PriceError::FetchFailed);
                    },
                    _ => {
                        return Err(PriceError::FetchFailed);
                    }
                }
            }

            // Parse response with enhanced error handling
            let response_text = response.text().await.map_err(|e| {
                warn!("Failed to read historical price response body: {}", e);
                PriceError::Network(e)
            })?;

            let historical_response: CoinGeckoHistoricalResponse = serde_json::from_str(&response_text)
                .map_err(|e| {
                    warn!("Failed to parse historical price JSON: {}", e);
                    warn!("Response body was: {}", response_text);
                    PriceError::InvalidData(format!("Failed to parse JSON: {}", e))
                })?;

            // Enhanced data availability checking
            if historical_response.prices.is_empty() {
                warn!("No historical price data available for {} hours ago", hours_ago);
                return Err(PriceError::InvalidData(format!(
                    "No historical price data available for {} hours ago", hours_ago
                )));
            }

            // Get the closest price data point
            let price_data = historical_response
                .prices
                .first()
                .ok_or_else(|| {
                    warn!("Empty price data array for {} hours ago", hours_ago);
                    PriceError::InvalidData("Empty historical price data array".to_string())
                })?;

            // Validate array structure
            if price_data.len() != 2 {
                return Err(PriceError::InvalidData(format!(
                    "Invalid price data format: expected [timestamp, price], got array of length {}",
                    price_data.len()
                )));
            }

            let price = price_data[1];
            let timestamp_ms = price_data[0] as i64;

            // Validate price data integrity (requirement 4.4)
            self.validate_price_data(price, "Historical price")?;

            // Enhanced timestamp validation
            let timestamp = DateTime::from_timestamp_millis(timestamp_ms)
                .ok_or_else(|| {
                    warn!("Invalid timestamp in historical data: {}", timestamp_ms);
                    PriceError::InvalidData(format!(
                        "Invalid timestamp in historical data: {}", timestamp_ms
                    ))
                })?;

            // Validate timestamp is reasonable (not too far in the future or past)
            let now = Utc::now();
            if timestamp > now {
                warn!("Historical timestamp is in the future: {}", timestamp);
                return Err(PriceError::ValidationFailed(
                    "Historical timestamp cannot be in the future".to_string()
                ));
            }

            // Check if timestamp is approximately what we requested
            let expected_time = now - Duration::hours(hours_ago as i64);
            let time_diff = (timestamp - expected_time).num_minutes().abs();
            if time_diff > 120 { // More than 2 hours difference
                warn!(
                    "Historical timestamp differs significantly from requested time: got {}, expected around {}",
                    timestamp, expected_time
                );
            }

            debug!(
                "Successfully fetched historical price: ${} at {} ({} hours ago)",
                price, timestamp, hours_ago
            );

            Ok(PriceData {
                current_price: price,
                timestamp,
            })
        }).await
    }

    /// Check if price is within the specified range
    pub fn is_price_in_range(&self, price: f64, range: &PriceRange) -> bool {
        range.contains(price)
    }

    /// Get price data with fallback strategies for enhanced reliability
    /// This method implements graceful degradation when primary data sources fail
    pub async fn get_price_with_fallback(&self, hours_ago: Option<u32>) -> Result<PriceData, PriceError> {
        match hours_ago {
            Some(hours) => {
                // Try historical price first
                match self.get_historical_price(hours).await {
                    Ok(data) => {
                        debug!("Successfully retrieved historical price for {} hours ago", hours);
                        Ok(data)
                    },
                    Err(historical_error) => {
                        warn!("Historical price failed, attempting current price as fallback: {}", historical_error);
                        
                        // Fallback to current price if historical fails
                        match self.get_current_price().await {
                            Ok(current_data) => {
                                warn!("Using current price as fallback for historical data");
                                Ok(current_data)
                            },
                            Err(current_error) => {
                                error!("Both historical and current price fetching failed");
                                error!("Historical error: {}", historical_error);
                                error!("Current price error: {}", current_error);
                                
                                // Return the more specific error
                                Err(historical_error)
                            }
                        }
                    }
                }
            },
            None => {
                // Just get current price
                self.get_current_price().await
            }
        }
    }

    /// Health check method to verify API connectivity
    /// Useful for system monitoring and diagnostics
    pub async fn health_check(&self) -> Result<(), PriceError> {
        debug!("Performing price monitor health check");
        
        // Try to fetch current price as a health check
        match self.get_current_price().await {
            Ok(data) => {
                debug!("Health check passed - current price: ${}", data.current_price);
                Ok(())
            },
            Err(e) => {
                warn!("Health check failed: {}", e);
                Err(e)
            }
        }
    }
}