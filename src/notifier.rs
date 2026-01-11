use crate::config::Config;
use crate::error::{BotError, NotificationError};
use crate::rebalancer::PositionMetrics;
use chrono::{DateTime, Utc};
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventType {
    Rebalance,
    DailyReport,
    Emergency,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationData {
    pub event_type: EventType,
    pub metrics: PositionMetrics,
    pub timestamp: DateTime<Utc>,
}

pub struct LineNotifier {
    client: Client,
    channel_access_token: String,
    user_id: String,
}

impl LineNotifier {
    pub async fn new(config: &Config) -> Result<Self, BotError> {
        let client = Client::new();
        
        Ok(LineNotifier {
            client,
            channel_access_token: config.line_channel_token.clone(),
            user_id: config.line_user_id.clone(),
        })
    }
    
    pub async fn send_rebalance_notification(&self, data: &NotificationData) -> Result<(), NotificationError> {
        let message = self.format_rebalance_message(data);
        self.send_message(&message).await
    }
    
    pub async fn send_daily_report(&self, data: &NotificationData) -> Result<(), NotificationError> {
        let message = self.format_daily_report_message(data);
        self.send_message(&message).await
    }
    
    pub async fn send_emergency_alert(&self, error: &BotError) -> Result<(), NotificationError> {
        let message = format!("🚨 Emergency Alert 🚨\n\nError: {}\nTime: {}", error, Utc::now().format("%Y-%m-%d %H:%M:%S UTC"));
        self.send_message(&message).await
    }
    
    fn format_rebalance_message(&self, data: &NotificationData) -> String {
        format!(
            "🔄 Position Rebalanced\n\n💰 Total Balance: ${:.2}\n📊 SOL: {:.4} | USDC: {:.2}\n💎 Yield Collected: ${:.2}\n📈 SOL Price: ${:.2}\n⏰ Time: {}",
            data.metrics.total_balance_usd,
            data.metrics.sol_amount,
            data.metrics.usdc_amount,
            data.metrics.collected_yield,
            data.metrics.sol_price,
            data.timestamp.format("%Y-%m-%d %H:%M:%S UTC")
        )
    }
    
    fn format_daily_report_message(&self, data: &NotificationData) -> String {
        format!(
            "📊 Daily Report\n\n💰 Total Balance: ${:.2}\n📊 SOL: {:.4} | USDC: {:.2}\n💎 Daily Yield: ${:.2}\n📈 SOL Price: ${:.2}\n⏰ Time: {}",
            data.metrics.total_balance_usd,
            data.metrics.sol_amount,
            data.metrics.usdc_amount,
            data.metrics.collected_yield,
            data.metrics.sol_price,
            data.timestamp.format("%Y-%m-%d %H:%M:%S UTC")
        )
    }
    
    async fn send_message(&self, message: &str) -> Result<(), NotificationError> {
        // TODO: Implement actual LINE API call with retry logic
        println!("LINE Message: {}", message); // Placeholder for development
        Ok(())
    }
}