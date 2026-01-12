use crate::error::ConfigError;
use serde::{Deserialize, Serialize};
use solana_sdk::pubkey::Pubkey;
use std::env;
use std::str::FromStr;
use std::time::Duration;
use url::Url;

#[derive(Clone, Serialize, Deserialize)]
pub struct Config {
    pub solana_rpc_url: String,
    pub whirlpool_program_id: Pubkey,
    pub position_address: Pubkey,
    #[serde(skip_serializing)]
    pub wallet_private_key: String,
    #[serde(skip_serializing)]
    pub line_channel_token: String,
    pub line_user_id: String,
    pub monitoring_interval: Duration,
    pub google_project_id: String,
    pub google_credentials_path: Option<String>,
}

impl std::fmt::Debug for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Config")
            .field("solana_rpc_url", &self.solana_rpc_url)
            .field("whirlpool_program_id", &self.whirlpool_program_id)
            .field("position_address", &self.position_address)
            .field("wallet_private_key", &"[REDACTED]")
            .field("line_channel_token", &"[REDACTED]")
            .field("line_user_id", &self.line_user_id)
            .field("monitoring_interval", &self.monitoring_interval)
            .field("google_project_id", &self.google_project_id)
            .field("google_credentials_path", &self.google_credentials_path)
            .finish()
    }
}

#[derive(Clone)]
pub struct SecretConfig {
    pub wallet_private_key: String,
    pub line_channel_token: String,
}

impl std::fmt::Debug for SecretConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SecretConfig")
            .field("wallet_private_key", &"[REDACTED]")
            .field("line_channel_token", &"[REDACTED]")
            .finish()
    }
}

impl std::fmt::Display for SecretConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SecretConfig {{ wallet_private_key: [REDACTED], line_channel_token: [REDACTED] }}")
    }
}

impl SecretConfig {
    /// Securely clear sensitive data from memory
    pub fn clear(&mut self) {
        self.wallet_private_key = String::new();
        self.line_channel_token = String::new();
        log::debug!("SecretConfig data cleared from memory");
    }
}

impl Config {
    pub async fn load_from_env() -> Result<Self, ConfigError> {
        let solana_rpc_url = env::var("SOLANA_RPC_URL")
            .unwrap_or_else(|_| "https://api.mainnet-beta.solana.com".to_string());

        let whirlpool_program_id = env::var("WHIRLPOOL_PROGRAM_ID")
            .map_err(|_| ConfigError::MissingConfig("WHIRLPOOL_PROGRAM_ID".to_string()))?;
        let whirlpool_program_id = Pubkey::from_str(&whirlpool_program_id)
            .map_err(|_| ConfigError::InvalidValue("WHIRLPOOL_PROGRAM_ID must be a valid Pubkey".to_string()))?;

        let position_address = env::var("POSITION_ADDRESS")
            .map_err(|_| ConfigError::MissingConfig("POSITION_ADDRESS".to_string()))?;
        let position_address = Pubkey::from_str(&position_address)
            .map_err(|_| ConfigError::InvalidValue("POSITION_ADDRESS must be a valid Pubkey".to_string()))?;

        let wallet_private_key = env::var("WALLET_PRIVATE_KEY")
            .unwrap_or_else(|_| String::new()); // Will be loaded from secrets if empty

        let line_channel_token = env::var("LINE_CHANNEL_TOKEN")
            .unwrap_or_else(|_| String::new()); // Will be loaded from secrets if empty

        let line_user_id = env::var("LINE_USER_ID")
            .map_err(|_| ConfigError::MissingConfig("LINE_USER_ID".to_string()))?;

        let monitoring_interval_secs: u64 = env::var("MONITORING_INTERVAL")
            .unwrap_or_else(|_| "3600".to_string()) // Default 1 hour
            .parse()
            .map_err(|_| ConfigError::InvalidValue("MONITORING_INTERVAL must be a number".to_string()))?;

        let google_project_id = env::var("GOOGLE_PROJECT_ID")
            .map_err(|_| ConfigError::MissingConfig("GOOGLE_PROJECT_ID".to_string()))?;

        let google_credentials_path = env::var("GOOGLE_APPLICATION_CREDENTIALS").ok();

        let config = Config {
            solana_rpc_url,
            whirlpool_program_id,
            position_address,
            wallet_private_key,
            line_channel_token,
            line_user_id,
            monitoring_interval: Duration::from_secs(monitoring_interval_secs),
            google_project_id,
            google_credentials_path,
        };
        
        config.validate()?;
        Ok(config)
    }

    pub async fn load_secrets(&self) -> Result<SecretConfig, ConfigError> {
        // If secrets are already provided via environment variables, use them
        if !self.wallet_private_key.is_empty() && !self.line_channel_token.is_empty() {
            return Ok(SecretConfig {
                wallet_private_key: self.wallet_private_key.clone(),
                line_channel_token: self.line_channel_token.clone(),
            });
        }

        // Otherwise, load from Google Secret Manager
        self.load_secrets_from_gcp().await
    }

    async fn load_secrets_from_gcp(&self) -> Result<SecretConfig, ConfigError> {
        use google_secretmanager1::SecretManager;
        use yup_oauth2;
        use hyper;
        use hyper_rustls;

        // Create authenticator
        let secret = if let Some(creds_path) = &self.google_credentials_path {
            yup_oauth2::read_service_account_key(creds_path)
                .await
                .map_err(|e| ConfigError::SecretRetrievalFailed(format!("Failed to read service account key: {}", e)))?
        } else {
            // Try to use default credentials
            return Err(ConfigError::SecretRetrievalFailed(
                "No Google credentials found. Set GOOGLE_APPLICATION_CREDENTIALS".to_string()
            ));
        };

        let auth = yup_oauth2::ServiceAccountAuthenticator::builder(secret)
            .build()
            .await
            .map_err(|e| ConfigError::SecretRetrievalFailed(format!("Failed to create authenticator: {}", e)))?;

        // Create Secret Manager client
        let client = SecretManager::new(
            hyper::Client::builder().build(
                hyper_rustls::HttpsConnectorBuilder::new()
                    .with_native_roots()
                    .https_or_http()
                    .enable_http1()
                    .build()
            ),
            auth
        );

        // Retrieve wallet private key
        let wallet_private_key = if self.wallet_private_key.is_empty() {
            let secret_name = format!("projects/{}/secrets/wallet-private-key/versions/latest", self.google_project_id);
            let (_, secret_data) = client
                .projects()
                .secrets_versions_access(&secret_name)
                .doit()
                .await
                .map_err(|e| ConfigError::SecretRetrievalFailed(format!("Failed to retrieve wallet private key: {}", e)))?;
            
            String::from_utf8(secret_data.payload.unwrap_or_default().data.unwrap_or_default())
                .map_err(|e| ConfigError::SecretRetrievalFailed(format!("Invalid wallet private key format: {}", e)))?
        } else {
            self.wallet_private_key.clone()
        };

        // Retrieve LINE channel token
        let line_channel_token = if self.line_channel_token.is_empty() {
            let secret_name = format!("projects/{}/secrets/line-channel-token/versions/latest", self.google_project_id);
            let (_, secret_data) = client
                .projects()
                .secrets_versions_access(&secret_name)
                .doit()
                .await
                .map_err(|e| ConfigError::SecretRetrievalFailed(format!("Failed to retrieve LINE channel token: {}", e)))?;
            
            String::from_utf8(secret_data.payload.unwrap_or_default().data.unwrap_or_default())
                .map_err(|e| ConfigError::SecretRetrievalFailed(format!("Invalid LINE channel token format: {}", e)))?
        } else {
            self.line_channel_token.clone()
        };

        Ok(SecretConfig {
            wallet_private_key,
            line_channel_token,
        })
    }

    /// Securely clear sensitive configuration data from memory
    pub fn clear_sensitive_data(&mut self) {
        // Replace sensitive strings with empty strings
        // The original data will be garbage collected
        self.wallet_private_key = String::new();
        self.line_channel_token = String::new();
        
        log::debug!("Sensitive configuration data cleared from memory");
    }
    
    pub fn validate(&self) -> Result<(), ConfigError> {
        // Validate Solana RPC URL
        if self.solana_rpc_url.is_empty() {
            return Err(ConfigError::InvalidValue("SOLANA_RPC_URL cannot be empty".to_string()));
        }
        
        let parsed_url = Url::parse(&self.solana_rpc_url)
            .map_err(|e| ConfigError::InvalidValue(format!("SOLANA_RPC_URL must be a valid URL: {}", e)))?;
        
        // Ensure it's HTTPS for security (except localhost for development)
        if parsed_url.scheme() != "https" && !parsed_url.host_str().unwrap_or("").contains("localhost") {
            return Err(ConfigError::InvalidValue(
                "SOLANA_RPC_URL must use HTTPS for security (except localhost)".to_string()
            ));
        }

        // Validate LINE user ID format (should be alphanumeric)
        if self.line_user_id.is_empty() {
            return Err(ConfigError::InvalidValue("LINE_USER_ID cannot be empty".to_string()));
        }
        
        if !self.line_user_id.chars().all(|c| c.is_alphanumeric()) {
            return Err(ConfigError::InvalidValue(
                "LINE_USER_ID must contain only alphanumeric characters".to_string()
            ));
        }
        
        // Validate monitoring interval (should be reasonable - between 1 minute and 24 hours)
        let interval_secs = self.monitoring_interval.as_secs();
        if interval_secs == 0 {
            return Err(ConfigError::InvalidValue("MONITORING_INTERVAL must be greater than 0".to_string()));
        }
        
        if interval_secs < 60 {
            return Err(ConfigError::InvalidValue(
                "MONITORING_INTERVAL must be at least 60 seconds for practical operation".to_string()
            ));
        }
        
        if interval_secs > 86400 {
            return Err(ConfigError::InvalidValue(
                "MONITORING_INTERVAL must be less than 24 hours (86400 seconds)".to_string()
            ));
        }

        // Validate Google project ID format
        if self.google_project_id.is_empty() {
            return Err(ConfigError::InvalidValue("GOOGLE_PROJECT_ID cannot be empty".to_string()));
        }
        
        // Google project IDs must be 6-30 characters, lowercase letters, digits, and hyphens
        if self.google_project_id.len() < 6 || self.google_project_id.len() > 30 {
            return Err(ConfigError::InvalidValue(
                "GOOGLE_PROJECT_ID must be between 6 and 30 characters".to_string()
            ));
        }
        
        if !self.google_project_id.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-') {
            return Err(ConfigError::InvalidValue(
                "GOOGLE_PROJECT_ID must contain only lowercase letters, digits, and hyphens".to_string()
            ));
        }
        
        if self.google_project_id.starts_with('-') || self.google_project_id.ends_with('-') {
            return Err(ConfigError::InvalidValue(
                "GOOGLE_PROJECT_ID cannot start or end with a hyphen".to_string()
            ));
        }

        // Validate credentials path if provided
        if let Some(creds_path) = &self.google_credentials_path {
            if !std::path::Path::new(creds_path).exists() {
                return Err(ConfigError::InvalidValue(
                    format!("Google credentials file does not exist: {}", creds_path)
                ));
            }
        }
        
        log::info!("Configuration validation passed");
        Ok(())
    }

    pub async fn reload_from_env(&mut self) -> Result<(), ConfigError> {
        log::info!("Attempting to reload configuration from environment");
        
        let old_config = self.clone();
        let new_config = Self::load_from_env().await?;
        
        // Check what changed
        let mut changes = Vec::new();
        
        if old_config.solana_rpc_url != new_config.solana_rpc_url {
            changes.push("SOLANA_RPC_URL");
        }
        if old_config.whirlpool_program_id != new_config.whirlpool_program_id {
            changes.push("WHIRLPOOL_PROGRAM_ID");
        }
        if old_config.position_address != new_config.position_address {
            changes.push("POSITION_ADDRESS");
        }
        if old_config.line_user_id != new_config.line_user_id {
            changes.push("LINE_USER_ID");
        }
        if old_config.monitoring_interval != new_config.monitoring_interval {
            changes.push("MONITORING_INTERVAL");
        }
        if old_config.google_project_id != new_config.google_project_id {
            changes.push("GOOGLE_PROJECT_ID");
        }
        if old_config.google_credentials_path != new_config.google_credentials_path {
            changes.push("GOOGLE_APPLICATION_CREDENTIALS");
        }
        
        if changes.is_empty() {
            log::info!("No configuration changes detected");
            return Ok(());
        }
        
        log::info!("Configuration changes detected: {}", changes.join(", "));
        
        // Apply the new configuration
        *self = new_config;
        
        log::info!("Configuration reloaded successfully with {} changes", changes.len());
        Ok(())
    }

    /// Validate secrets configuration
    pub async fn validate_secrets(&self) -> Result<(), ConfigError> {
        log::info!("Validating secrets configuration");
        
        let secrets = self.load_secrets().await?;
        
        // Validate wallet private key format (should be base58 encoded)
        if secrets.wallet_private_key.is_empty() {
            return Err(ConfigError::InvalidValue("Wallet private key cannot be empty".to_string()));
        }
        
        // Basic validation - should be reasonable length for a private key
        if secrets.wallet_private_key.len() < 32 || secrets.wallet_private_key.len() > 128 {
            return Err(ConfigError::InvalidValue(
                "Wallet private key has invalid length (expected 32-128 characters)".to_string()
            ));
        }
        
        // Validate LINE channel token format
        if secrets.line_channel_token.is_empty() {
            return Err(ConfigError::InvalidValue("LINE channel token cannot be empty".to_string()));
        }
        
        // LINE channel access tokens are typically long strings
        if secrets.line_channel_token.len() < 50 {
            return Err(ConfigError::InvalidValue(
                "LINE channel token appears to be too short (expected at least 50 characters)".to_string()
            ));
        }
        
        log::info!("Secrets validation passed");
        Ok(())
    }
}