use thiserror::Error;

/// Main bot error type that encompasses all possible errors
#[derive(Debug, Error)]
pub enum BotError {
    #[error("Price monitoring error: {0}")]
    Price(#[from] PriceError),
    #[error("Position management error: {0}")]
    Position(#[from] PositionError),
    #[error("Notification error: {0}")]
    Notification(#[from] NotificationError),
    #[error("State management error: {0}")]
    State(#[from] StateError),
    #[error("Configuration error: {0}")]
    Config(String),
}

/// Price monitoring related errors
#[derive(Debug, Error)]
pub enum PriceError {
    #[error("Failed to fetch price data")]
    FetchFailed,
    #[error("Invalid price data: {0}")]
    InvalidData(String),
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),
    #[error("Price data validation failed: {0}")]
    ValidationFailed(String),
}

/// Position management related errors
#[derive(Debug, Error)]
pub enum PositionError {
    #[error("Failed to interact with Orca protocol: {0}")]
    OrcaInteractionFailed(String),
    #[error("Insufficient funds for operation")]
    InsufficientFunds,
    #[error("Transaction failed: {0}")]
    TransactionFailed(String),
    #[error("Position not found: {0}")]
    PositionNotFound(String),
    #[error("Invalid position parameters: {0}")]
    InvalidParameters(String),
}

/// LINE notification related errors
#[derive(Debug, Error)]
pub enum NotificationError {
    #[error("Failed to send LINE notification: {0}")]
    SendFailed(String),
    #[error("Invalid LINE configuration: {0}")]
    InvalidConfig(String),
    #[error("Message formatting error: {0}")]
    FormattingError(String),
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),
}

/// State management related errors
#[derive(Debug, Error)]
pub enum StateError {
    #[error("Failed to load state: {0}")]
    LoadFailed(String),
    #[error("Failed to save state: {0}")]
    SaveFailed(String),
    #[error("State file not found: {0}")]
    FileNotFound(String),
    #[error("State serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}