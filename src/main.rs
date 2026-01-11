use log::info;
use std::env;

mod bot;
mod config;
mod error;
mod monitor;
mod notifier;
mod rebalancer;

use crate::bot::OrcaBot;
use crate::config::Config;
use crate::error::BotError;

#[tokio::main]
async fn main() -> Result<(), BotError> {
    // Initialize logging
    env_logger::init();
    
    info!("Starting Orca Liquidity Bot");
    
    // Load configuration
    let config = Config::load_from_env().await?;
    
    // Initialize bot
    let bot = OrcaBot::new(config).await?;
    
    info!("Orca Liquidity Bot initialized successfully");
    
    // TODO: Add HTTP server for Cloud Run deployment
    // TODO: Add scheduled task handlers
    
    Ok(())
}