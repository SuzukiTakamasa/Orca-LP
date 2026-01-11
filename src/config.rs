use crate::error::ConfigError;
use chrono::Duration;
use serde::{Deserialize, Serialize};
use solana_client::rpc_client::RpcClient;
use std::env;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub solana_rpc_url: String,
    pub whirlpool_program_id: String,
    pub position_address: String,
    pub wallet_private_key: String,
    pub line_channel_token: String,
    pub line_user_id: String,
    pub monitoring_interval: u64, // seconds
}

impl Config {
    pub async fn load_from_env() -> Result<Self, ConfigError> {
        let config = Config {
            solana_rpc_url: env::var("SOLANA_RPC_URL")
                .unwrap_or_else(|_| "https://api.mainnet-beta.solana.com".to_string()),
            whirlpool_program_id: env::var("WHIRLPOOL_PROGRAM_ID")
                .map_err(|_| ConfigError::MissingConfig("WHIRLPOOL_PROGRAM_ID".to_string()))?,
            position_address: env::var("POSITION_ADDRESS")
                .map_err(|_| ConfigError::MissingConfig("POSITION_ADDRESS".to_string()))?,
            wallet_private_key: env::var("WALLET_PRIVATE_KEY")
                .map_err(|_| ConfigError::MissingConfig("WALLET_PRIVATE_KEY".to_string()))?,
            line_channel_token: env::var("LINE_CHANNEL_TOKEN")
                .map_err(|_| ConfigError::MissingConfig("LINE_CHANNEL_TOKEN".to_string()))?,
            line_user_id: env::var("LINE_USER_ID")
                .map_err(|_| ConfigError::MissingConfig("LINE_USER_ID".to_string()))?,
            monitoring_interval: env::var("MONITORING_INTERVAL")
                .unwrap_or_else(|_| "3600".to_string()) // Default 1 hour
                .parse()
                .map_err(|_| ConfigError::InvalidValue("MONITORING_INTERVAL must be a number".to_string()))?,
        };
        
        config.validate()?;
        Ok(config)
    }
    
    pub fn validate(&self) -> Result<(), ConfigError> {
        if self.solana_rpc_url.is_empty() {
            return Err(ConfigError::InvalidValue("SOLANA_RPC_URL cannot be empty".to_string()));
        }
        
        if self.whirlpool_program_id.is_empty() {
            return Err(ConfigError::InvalidValue("WHIRLPOOL_PROGRAM_ID cannot be empty".to_string()));
        }
        
        if self.position_address.is_empty() {
            return Err(ConfigError::InvalidValue("POSITION_ADDRESS cannot be empty".to_string()));
        }
        
        if self.wallet_private_key.is_empty() {
            return Err(ConfigError::InvalidValue("WALLET_PRIVATE_KEY cannot be empty".to_string()));
        }
        
        if self.line_channel_token.is_empty() {
            return Err(ConfigError::InvalidValue("LINE_CHANNEL_TOKEN cannot be empty".to_string()));
        }
        
        if self.line_user_id.is_empty() {
            return Err(ConfigError::InvalidValue("LINE_USER_ID cannot be empty".to_string()));
        }
        
        if self.monitoring_interval == 0 {
            return Err(ConfigError::InvalidValue("MONITORING_INTERVAL must be greater than 0".to_string()));
        }
        
        Ok(())
    }
}