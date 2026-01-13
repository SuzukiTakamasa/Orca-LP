use log::info;
use orca_liquidity_bot::{OrcaBot, Config, BotError};

#[tokio::main]
async fn main() -> Result<(), BotError> {
    // Initialize logging
    env_logger::init();
    
    info!("Starting Orca Liquidity Bot");
    
    // Load configuration
    let config = Config::load_from_env().await?;
    
    // Initialize bot
    let _bot = OrcaBot::new(config).await?;
    
    info!("Orca Liquidity Bot initialized successfully");
    
    // TODO: Add HTTP server for Cloud Run deployment
    // TODO: Add scheduled task handlers
    
    Ok(())
}