use orca_liquidity_bot::{load_config, BotError};
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), BotError> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    info!("Starting Orca Liquidity Bot");

    // Load configuration from environment variables
    let _config = load_config()?;
    
    info!("Configuration loaded successfully");
    
    // TODO: Initialize components and start HTTP server
    // This will be implemented in later tasks
    
    Ok(())
}
