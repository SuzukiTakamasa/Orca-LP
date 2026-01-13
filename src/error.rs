use thiserror::Error;

/// Main error type for the Orca Bot system
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
    
    #[error("Validation error: {0}")]
    Validation(String),
    
    #[error("System initialization error: {0}")]
    Initialization(String),
    
    #[error("Critical system error: {0}")]
    Critical(String),
    
    #[error("Recovery failed: {0}")]
    RecoveryFailed(String),
}

impl BotError {
    /// Creates a validation error
    pub fn validation(msg: impl Into<String>) -> Self {
        Self::Validation(msg.into())
    }
    
    /// Creates an initialization error
    pub fn initialization(msg: impl Into<String>) -> Self {
        Self::Initialization(msg.into())
    }
    
    /// Creates a critical error
    pub fn critical(msg: impl Into<String>) -> Self {
        Self::Critical(msg.into())
    }
    
    /// Creates a recovery failed error
    pub fn recovery_failed(msg: impl Into<String>) -> Self {
        Self::RecoveryFailed(msg.into())
    }
    
    /// Checks if this error is recoverable
    pub fn is_recoverable(&self) -> bool {
        match self {
            BotError::Price(e) => e.is_recoverable(),
            BotError::Rebalance(e) => e.is_recoverable(),
            BotError::Notification(e) => e.is_recoverable(),
            BotError::Config(_) => false, // Config errors are not recoverable
            BotError::Rpc(_) => true, // RPC errors are usually transient
            BotError::Validation(_) => false,
            BotError::Initialization(_) => false,
            BotError::Critical(_) => false,
            BotError::RecoveryFailed(_) => false,
        }
    }
    
    /// Checks if this error requires emergency notification
    pub fn requires_emergency_notification(&self) -> bool {
        match self {
            BotError::Critical(_) => true,
            BotError::RecoveryFailed(_) => true,
            BotError::Rebalance(RebalanceError::WalletAccessFailed(_)) => true,
            BotError::Config(ConfigError::SecretRetrievalFailed(_)) => true,
            _ => false,
        }
    }
}

/// Errors related to price monitoring and data retrieval
#[derive(Debug, Error)]
pub enum PriceError {
    #[error("Failed to retrieve price data: {0}")]
    RetrievalFailed(String),
    
    #[error("Invalid price data: {0}")]
    InvalidData(String),
    
    #[error("Price history error: {0}")]
    HistoryError(String),
    
    #[error("Price source unavailable: {0}")]
    SourceUnavailable(String),
    
    #[error("Price data too old: {0}")]
    DataTooOld(String),
    
    #[error("Network timeout while fetching price")]
    NetworkTimeout,
    
    #[error("Price parsing error: {0}")]
    ParsingError(String),
}

impl PriceError {
    /// Checks if this price error is recoverable
    pub fn is_recoverable(&self) -> bool {
        match self {
            PriceError::RetrievalFailed(_) => true,
            PriceError::SourceUnavailable(_) => true,
            PriceError::NetworkTimeout => true,
            PriceError::InvalidData(_) => false,
            PriceError::HistoryError(_) => false,
            PriceError::DataTooOld(_) => true,
            PriceError::ParsingError(_) => false,
        }
    }
}

/// Errors related to position rebalancing and yield collection
#[derive(Debug, Error)]
pub enum RebalanceError {
    #[error("Position operation failed: {0}")]
    PositionFailed(String),
    
    #[error("Yield collection failed: {0}")]
    YieldCollectionFailed(String),
    
    #[error("Range calculation failed: {0}")]
    RangeCalculationFailed(String),
    
    #[error("Wallet access failed: {0}")]
    WalletAccessFailed(String),
    
    #[error("Insufficient balance: {0}")]
    InsufficientBalance(String),
    
    #[error("Transaction failed: {0}")]
    TransactionFailed(String),
    
    #[error("Position not found: {0}")]
    PositionNotFound(String),
    
    #[error("Slippage tolerance exceeded: {0}")]
    SlippageExceeded(String),
    
    #[error("Orca SDK error: {0}")]
    OrcaSdkError(String),
}

impl RebalanceError {
    /// Checks if this rebalance error is recoverable
    pub fn is_recoverable(&self) -> bool {
        match self {
            RebalanceError::PositionFailed(_) => true,
            RebalanceError::YieldCollectionFailed(_) => true,
            RebalanceError::TransactionFailed(_) => true,
            RebalanceError::SlippageExceeded(_) => true,
            RebalanceError::OrcaSdkError(_) => true,
            RebalanceError::WalletAccessFailed(_) => false,
            RebalanceError::InsufficientBalance(_) => false,
            RebalanceError::PositionNotFound(_) => false,
            RebalanceError::RangeCalculationFailed(_) => false,
        }
    }
}

/// Errors related to LINE notifications
#[derive(Debug, Error)]
pub enum NotificationError {
    #[error("LINE API error: {0}")]
    LineApiError(String),
    
    #[error("Message formatting error: {0}")]
    FormattingError(String),
    
    #[error("Retry limit exceeded")]
    RetryLimitExceeded,
    
    #[error("Authentication failed: {0}")]
    AuthenticationFailed(String),
    
    #[error("Rate limit exceeded")]
    RateLimitExceeded,
    
    #[error("Network error: {0}")]
    NetworkError(String),
    
    #[error("Invalid recipient: {0}")]
    InvalidRecipient(String),
    
    #[error("Message too large: {0}")]
    MessageTooLarge(String),
}

impl NotificationError {
    /// Checks if this notification error is recoverable
    pub fn is_recoverable(&self) -> bool {
        match self {
            NotificationError::LineApiError(_) => true,
            NotificationError::NetworkError(_) => true,
            NotificationError::RateLimitExceeded => true,
            NotificationError::FormattingError(_) => false,
            NotificationError::RetryLimitExceeded => false,
            NotificationError::AuthenticationFailed(_) => false,
            NotificationError::InvalidRecipient(_) => false,
            NotificationError::MessageTooLarge(_) => false,
        }
    }
}

/// Errors related to configuration management
#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Missing required configuration: {0}")]
    MissingConfig(String),
    
    #[error("Invalid configuration value: {0}")]
    InvalidValue(String),
    
    #[error("Secret retrieval failed: {0}")]
    SecretRetrievalFailed(String),
    
    #[error("Environment variable error: {0}")]
    EnvironmentError(String),
    
    #[error("Configuration file error: {0}")]
    FileError(String),
    
    #[error("Configuration validation failed: {0}")]
    ValidationFailed(String),
    
    #[error("Hot reload failed: {0}")]
    HotReloadFailed(String),
    
    #[error("Google Secret Manager error: {0}")]
    SecretManagerError(String),
}

/// Result type alias for Bot operations
pub type BotResult<T> = Result<T, BotError>;

/// Result type alias for Price operations
pub type PriceResult<T> = Result<T, PriceError>;

/// Result type alias for Rebalance operations
pub type RebalanceResult<T> = Result<T, RebalanceError>;

/// Result type alias for Notification operations
pub type NotificationResult<T> = Result<T, NotificationError>;

/// Result type alias for Configuration operations
pub type ConfigResult<T> = Result<T, ConfigError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bot_error_recoverability() {
        // Test recoverable errors
        let price_error = BotError::Price(PriceError::NetworkTimeout);
        assert!(price_error.is_recoverable());

        // Test non-recoverable errors
        let config_error = BotError::Config(ConfigError::MissingConfig("test".to_string()));
        assert!(!config_error.is_recoverable());

        let critical_error = BotError::critical("system failure");
        assert!(!critical_error.is_recoverable());
    }

    #[test]
    fn test_emergency_notification_requirements() {
        // Test errors that require emergency notification
        let critical_error = BotError::critical("system failure");
        assert!(critical_error.requires_emergency_notification());

        let recovery_error = BotError::recovery_failed("recovery attempt failed");
        assert!(recovery_error.requires_emergency_notification());

        let wallet_error = BotError::Rebalance(RebalanceError::WalletAccessFailed("wallet locked".to_string()));
        assert!(wallet_error.requires_emergency_notification());

        // Test errors that don't require emergency notification
        let price_error = BotError::Price(PriceError::NetworkTimeout);
        assert!(!price_error.requires_emergency_notification());
    }

    #[test]
    fn test_price_error_recoverability() {
        // Recoverable price errors
        assert!(PriceError::RetrievalFailed("network issue".to_string()).is_recoverable());
        assert!(PriceError::NetworkTimeout.is_recoverable());
        assert!(PriceError::SourceUnavailable("api down".to_string()).is_recoverable());

        // Non-recoverable price errors
        assert!(!PriceError::InvalidData("malformed data".to_string()).is_recoverable());
        assert!(!PriceError::ParsingError("json error".to_string()).is_recoverable());
    }

    #[test]
    fn test_rebalance_error_recoverability() {
        // Recoverable rebalance errors
        assert!(RebalanceError::TransactionFailed("network congestion".to_string()).is_recoverable());
        assert!(RebalanceError::SlippageExceeded("high volatility".to_string()).is_recoverable());

        // Non-recoverable rebalance errors
        assert!(!RebalanceError::WalletAccessFailed("private key invalid".to_string()).is_recoverable());
        assert!(!RebalanceError::InsufficientBalance("not enough SOL".to_string()).is_recoverable());
    }

    #[test]
    fn test_notification_error_recoverability() {
        // Recoverable notification errors
        assert!(NotificationError::NetworkError("timeout".to_string()).is_recoverable());
        assert!(NotificationError::RateLimitExceeded.is_recoverable());

        // Non-recoverable notification errors
        assert!(!NotificationError::AuthenticationFailed("invalid token".to_string()).is_recoverable());
        assert!(!NotificationError::RetryLimitExceeded.is_recoverable());
    }

    #[test]
    fn test_error_creation_helpers() {
        let validation_error = BotError::validation("invalid input");
        match validation_error {
            BotError::Validation(msg) => assert_eq!(msg, "invalid input"),
            _ => panic!("Expected validation error"),
        }

        let critical_error = BotError::critical("system failure");
        match critical_error {
            BotError::Critical(msg) => assert_eq!(msg, "system failure"),
            _ => panic!("Expected critical error"),
        }
    }

    #[test]
    fn test_error_chaining() {
        let price_error = PriceError::NetworkTimeout;
        let bot_error = BotError::from(price_error);
        
        match bot_error {
            BotError::Price(PriceError::NetworkTimeout) => (),
            _ => panic!("Error chaining failed"),
        }
    }
}