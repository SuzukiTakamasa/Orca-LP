use thiserror::Error;

#[derive(Debug, Error)]
pub enum BotError {
    #[error("Price monitoring error: {0}")]
    Price(#[from] PriceError),
    
    #[error("Rebalancing error: {0}")]
    Rebalance(#[from] RebalanceError),
    
    #[error("Notification error: {0}")]
    Notification(#[from] NotificationError),
    
    #[error("Configuration error: {0}")]
    Config(#[from] ConfigError),
    
    #[error("Solana RPC error: {0}")]
    Rpc(#[from] solana_client::client_error::ClientError),
}

#[derive(Debug, Error)]
pub enum PriceError {
    #[error("Failed to retrieve price data: {0}")]
    RetrievalFailed(String),
    
    #[error("Invalid price data: {0}")]
    InvalidData(String),
    
    #[error("Price history error: {0}")]
    HistoryError(String),
}

#[derive(Debug, Error)]
pub enum RebalanceError {
    #[error("Position operation failed: {0}")]
    PositionFailed(String),
    
    #[error("Yield collection failed: {0}")]
    YieldCollectionFailed(String),
    
    #[error("Range calculation failed: {0}")]
    RangeCalculationFailed(String),
}

#[derive(Debug, Error)]
pub enum NotificationError {
    #[error("LINE API error: {0}")]
    LineApiError(String),
    
    #[error("Message formatting error: {0}")]
    FormattingError(String),
    
    #[error("Retry limit exceeded")]
    RetryLimitExceeded,
}

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Missing required configuration: {0}")]
    MissingConfig(String),
    
    #[error("Invalid configuration value: {0}")]
    InvalidValue(String),
    
    #[error("Secret retrieval failed: {0}")]
    SecretRetrievalFailed(String),
}