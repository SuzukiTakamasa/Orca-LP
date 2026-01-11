use crate::config::Config;
use crate::error::{BotError, PriceError};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use solana_client::rpc_client::RpcClient;
use std::collections::VecDeque;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceData {
    pub current_price: f64,
    pub timestamp: DateTime<Utc>,
    pub source: PriceSource,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PriceSource {
    Orca,
    Jupiter,
    Pyth,
}

#[derive(Debug, Clone)]
pub struct PriceHistory {
    pub entries: VecDeque<PriceData>,
    pub max_entries: usize,
}

impl PriceHistory {
    pub fn new(max_entries: usize) -> Self {
        Self {
            entries: VecDeque::new(),
            max_entries,
        }
    }
    
    pub fn add_entry(&mut self, price: PriceData) {
        if self.entries.len() >= self.max_entries {
            self.entries.pop_front();
        }
        self.entries.push_back(price);
    }
    
    pub fn get_price_at(&self, duration_ago: chrono::Duration) -> Option<&PriceData> {
        let target_time = Utc::now() - duration_ago;
        self.entries
            .iter()
            .rev()
            .find(|entry| entry.timestamp <= target_time)
    }
}

#[derive(Debug, Clone)]
pub struct Position {
    pub address: String,
    pub whirlpool: String,
    pub tick_lower: i32,
    pub tick_upper: i32,
    pub liquidity: u128,
}

#[derive(Debug, Clone)]
pub struct PriceRange {
    pub lower_price: f64,
    pub upper_price: f64,
    pub lower_tick: i32,
    pub upper_tick: i32,
}

pub struct PriceMonitor {
    rpc_client: RpcClient,
    price_history: PriceHistory,
}

impl PriceMonitor {
    pub async fn new(config: &Config) -> Result<Self, BotError> {
        let rpc_client = RpcClient::new(&config.solana_rpc_url);
        let price_history = PriceHistory::new(168); // Store 1 week of hourly data
        
        Ok(PriceMonitor {
            rpc_client,
            price_history,
        })
    }
    
    pub async fn get_current_price(&self) -> Result<PriceData, PriceError> {
        // TODO: Implement price retrieval from Orca/Jupiter
        Ok(PriceData {
            current_price: 100.0, // Placeholder
            timestamp: Utc::now(),
            source: PriceSource::Orca,
        })
    }
    
    pub async fn check_range_deviation(&self, position: &Position) -> Result<bool, PriceError> {
        // TODO: Implement range deviation check
        Ok(false)
    }
    
    pub async fn store_price_history(&mut self, price: PriceData) -> Result<(), PriceError> {
        self.price_history.add_entry(price);
        Ok(())
    }
}