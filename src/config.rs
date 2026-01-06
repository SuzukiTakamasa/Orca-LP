use crate::{BotConfig, BotError};
use std::env;

/// Load configuration from environment variables
pub fn load_config() -> Result<BotConfig, BotError> {
    let line_channel_access_token = env::var("LINE_CHANNEL_ACCESS_TOKEN")
        .map_err(|_| BotError::Config("LINE_CHANNEL_ACCESS_TOKEN not set".to_string()))?;
    
    let line_user_id = env::var("LINE_USER_ID")
        .map_err(|_| BotError::Config("LINE_USER_ID not set".to_string()))?;
    
    let wallet_private_key = env::var("WALLET_PRIVATE_KEY")
        .map_err(|_| BotError::Config("WALLET_PRIVATE_KEY not set".to_string()))?;
    
    let rpc_endpoint = env::var("RPC_ENDPOINT")
        .unwrap_or_else(|_| "https://api.mainnet-beta.solana.com".to_string());
    
    let state_file_path = env::var("STATE_FILE_PATH")
        .unwrap_or_else(|_| "/tmp/bot_state.json".to_string());

    // Validate configuration
    validate_config(&BotConfig {
        line_channel_access_token: line_channel_access_token.clone(),
        line_user_id: line_user_id.clone(),
        wallet_private_key: wallet_private_key.clone(),
        rpc_endpoint: rpc_endpoint.clone(),
        state_file_path: state_file_path.clone(),
    })?;

    Ok(BotConfig {
        line_channel_access_token,
        line_user_id,
        wallet_private_key,
        rpc_endpoint,
        state_file_path,
    })
}

/// Validate configuration values
pub fn validate_config(config: &BotConfig) -> Result<(), BotError> {
    // Validate LINE channel access token
    if config.line_channel_access_token.is_empty() {
        return Err(BotError::Config("LINE_CHANNEL_ACCESS_TOKEN cannot be empty".to_string()));
    }

    // Validate LINE user ID
    if config.line_user_id.is_empty() {
        return Err(BotError::Config("LINE_USER_ID cannot be empty".to_string()));
    }

    // Validate wallet private key (basic format check)
    if config.wallet_private_key.is_empty() {
        return Err(BotError::Config("WALLET_PRIVATE_KEY cannot be empty".to_string()));
    }

    // Validate RPC endpoint (basic URL format check)
    if config.rpc_endpoint.is_empty() {
        return Err(BotError::Config("RPC_ENDPOINT cannot be empty".to_string()));
    }
    
    if !config.rpc_endpoint.starts_with("http://") && !config.rpc_endpoint.starts_with("https://") {
        return Err(BotError::Config("RPC_ENDPOINT must be a valid HTTP/HTTPS URL".to_string()));
    }

    // Validate state file path
    if config.state_file_path.is_empty() {
        return Err(BotError::Config("STATE_FILE_PATH cannot be empty".to_string()));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    use std::collections::HashMap;

    // Helper function to create a valid config for testing
    fn create_valid_config() -> BotConfig {
        BotConfig {
            line_channel_access_token: "valid_token".to_string(),
            line_user_id: "U1234567890".to_string(),
            wallet_private_key: "5J1F7GHaLNjmQBbmHPdcvPL9aRoKrhgzWWn4hs9urviDvLEKK".to_string(),
            rpc_endpoint: "https://api.mainnet-beta.solana.com".to_string(),
            state_file_path: "/tmp/bot_state.json".to_string(),
        }
    }

    // Helper function to set environment variables for testing
    fn set_test_env_vars(vars: HashMap<&str, &str>) {
        for (key, value) in vars {
            env::set_var(key, value);
        }
    }

    // Helper function to clear environment variables for testing
    fn clear_test_env_vars(keys: &[&str]) {
        for key in keys {
            env::remove_var(key);
        }
    }

    #[test]
    fn test_validate_config_valid() {
        let config = create_valid_config();
        assert!(validate_config(&config).is_ok());
    }

    #[test]
    fn test_validate_config_empty_line_token() {
        let mut config = create_valid_config();
        config.line_channel_access_token = "".to_string();
        
        let result = validate_config(&config);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("LINE_CHANNEL_ACCESS_TOKEN cannot be empty"));
    }

    #[test]
    fn test_validate_config_empty_line_user_id() {
        let mut config = create_valid_config();
        config.line_user_id = "".to_string();
        
        let result = validate_config(&config);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("LINE_USER_ID cannot be empty"));
    }

    #[test]
    fn test_validate_config_empty_wallet_key() {
        let mut config = create_valid_config();
        config.wallet_private_key = "".to_string();
        
        let result = validate_config(&config);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("WALLET_PRIVATE_KEY cannot be empty"));
    }

    #[test]
    fn test_validate_config_invalid_rpc_endpoint() {
        let mut config = create_valid_config();
        config.rpc_endpoint = "invalid-url".to_string();
        
        let result = validate_config(&config);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("RPC_ENDPOINT must be a valid HTTP/HTTPS URL"));
    }

    #[test]
    fn test_validate_config_empty_state_file_path() {
        let mut config = create_valid_config();
        config.state_file_path = "".to_string();
        
        let result = validate_config(&config);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("STATE_FILE_PATH cannot be empty"));
    }

    #[test]
    fn test_load_config_success() {
        let env_vars = HashMap::from([
            ("LINE_CHANNEL_ACCESS_TOKEN", "test_token"),
            ("LINE_USER_ID", "U1234567890"),
            ("WALLET_PRIVATE_KEY", "test_private_key"),
            ("RPC_ENDPOINT", "https://api.devnet.solana.com"),
            ("STATE_FILE_PATH", "/tmp/test_state.json"),
        ]);
        
        set_test_env_vars(env_vars);
        
        let result = load_config();
        assert!(result.is_ok());
        
        let config = result.unwrap();
        assert_eq!(config.line_channel_access_token, "test_token");
        assert_eq!(config.line_user_id, "U1234567890");
        assert_eq!(config.wallet_private_key, "test_private_key");
        assert_eq!(config.rpc_endpoint, "https://api.devnet.solana.com");
        assert_eq!(config.state_file_path, "/tmp/test_state.json");
        
        // Clean up
        clear_test_env_vars(&["LINE_CHANNEL_ACCESS_TOKEN", "LINE_USER_ID", "WALLET_PRIVATE_KEY", "RPC_ENDPOINT", "STATE_FILE_PATH"]);
    }

    #[test]
    fn test_load_config_missing_required_env() {
        // Clear all environment variables
        clear_test_env_vars(&["LINE_CHANNEL_ACCESS_TOKEN", "LINE_USER_ID", "WALLET_PRIVATE_KEY"]);
        
        let result = load_config();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("LINE_CHANNEL_ACCESS_TOKEN not set"));
    }

    // Property-based test for configuration validation
    // Feature: orca-liquidity-bot, Property 15: Configuration validation
    proptest! {
        #[test]
        fn property_config_validation_comprehensive(
            line_token in ".*",
            line_user_id in ".*",
            wallet_key in ".*",
            rpc_endpoint in ".*",
            state_path in ".*"
        ) {
            let config = BotConfig {
                line_channel_access_token: line_token.clone(),
                line_user_id: line_user_id.clone(),
                wallet_private_key: wallet_key.clone(),
                rpc_endpoint: rpc_endpoint.clone(),
                state_file_path: state_path.clone(),
            };

            let validation_result = validate_config(&config);

            // Property: Configuration validation should fail if any required field is empty
            // or if RPC endpoint doesn't have proper URL format
            let should_be_valid = !line_token.is_empty() 
                && !line_user_id.is_empty() 
                && !wallet_key.is_empty() 
                && !rpc_endpoint.is_empty()
                && (rpc_endpoint.starts_with("http://") || rpc_endpoint.starts_with("https://"))
                && !state_path.is_empty();

            if should_be_valid {
                prop_assert!(validation_result.is_ok(), 
                    "Expected valid config to pass validation, but got error: {:?}", 
                    validation_result.err());
            } else {
                prop_assert!(validation_result.is_err(), 
                    "Expected invalid config to fail validation, but it passed");
            }
        }
    }

    proptest! {
        #[test]
        fn property_config_validation_url_format(
            protocol in prop::sample::select(vec!["http://", "https://", "ftp://", "invalid://", ""]),
            domain in "[a-zA-Z0-9.-]+",
            path in "[a-zA-Z0-9/._-]*"
        ) {
            let rpc_endpoint = format!("{}{}{}", protocol, domain, path);
            
            let config = BotConfig {
                line_channel_access_token: "valid_token".to_string(),
                line_user_id: "valid_user".to_string(),
                wallet_private_key: "valid_key".to_string(),
                rpc_endpoint: rpc_endpoint.clone(),
                state_file_path: "/valid/path".to_string(),
            };

            let validation_result = validate_config(&config);

            // Property: RPC endpoint validation should only accept HTTP/HTTPS URLs
            let is_valid_url = rpc_endpoint.starts_with("http://") || rpc_endpoint.starts_with("https://");
            
            if is_valid_url && !domain.is_empty() {
                prop_assert!(validation_result.is_ok(), 
                    "Expected valid HTTP/HTTPS URL to pass validation: {}", rpc_endpoint);
            } else {
                prop_assert!(validation_result.is_err(), 
                    "Expected invalid URL format to fail validation: {}", rpc_endpoint);
            }
        }
    }

    proptest! {
        #[test]
        fn property_config_validation_empty_fields(
            field_to_empty in 0..5usize
        ) {
            let mut config = create_valid_config();
            
            // Empty one field based on the generated index
            match field_to_empty {
                0 => config.line_channel_access_token = "".to_string(),
                1 => config.line_user_id = "".to_string(),
                2 => config.wallet_private_key = "".to_string(),
                3 => config.rpc_endpoint = "".to_string(),
                4 => config.state_file_path = "".to_string(),
                _ => unreachable!(),
            }

            let validation_result = validate_config(&config);

            // Property: Configuration validation should fail if any required field is empty
            prop_assert!(validation_result.is_err(), 
                "Expected configuration with empty field {} to fail validation", field_to_empty);
        }
    }
}