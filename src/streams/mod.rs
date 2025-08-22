mod tests;
mod client;
mod connection;
pub mod events;
pub mod specs;

pub use client::BinanceSpotStreamClient;

use crate::Result;
use crate::{
    BinanceConfig, 
    StreamConfig
};


pub fn client(config: BinanceConfig<StreamConfig>) -> Result<BinanceSpotStreamClient> {
    BinanceSpotStreamClient::new(config)
}