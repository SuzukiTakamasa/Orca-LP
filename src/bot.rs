use crate::config::Config;
use crate::error::BotError;
use crate::monitor::PriceMonitor;
use crate::notifier::LineNotifier;
use crate::rebalancer::Rebalancer;
use log::{error, info};

pub struct OrcaBot {
    price_monitor: PriceMonitor,
    rebalancer: Rebalancer,
    notifier: LineNotifier,
    config: Config,
}

impl OrcaBot {
    pub async fn new(config: Config) -> Result<Self, BotError> {
        info!("Initializing Orca Bot components");
        
        let price_monitor = PriceMonitor::new(&config).await?;
        let rebalancer = Rebalancer::new(&config).await?;
        let notifier = LineNotifier::new(&config).await?;
        
        Ok(OrcaBot {
            price_monitor,
            rebalancer,
            notifier,
            config,
        })
    }
    
    pub async fn run_hourly_check(&self) -> Result<(), BotError> {
        info!("Running hourly price check");
        // TODO: Implement hourly check logic
        Ok(())
    }
    
    pub async fn run_daily_collection(&self) -> Result<(), BotError> {
        info!("Running daily yield collection");
        // TODO: Implement daily collection logic
        Ok(())
    }
    
    pub async fn handle_emergency(&self, error: &BotError) -> Result<(), BotError> {
        error!("Handling emergency: {}", error);
        // TODO: Implement emergency handling logic
        Ok(())
    }
}