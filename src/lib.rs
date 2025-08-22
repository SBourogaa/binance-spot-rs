pub type Result<T> = anyhow::Result<T>;

mod auth;
pub mod enums;
pub mod types;
mod errors;
mod filters;
pub mod clients;
mod config;
pub mod streams;

pub use auth::Ed25519Signer;
pub use errors::BinanceError;
pub use config::{BinanceConfig, WebSocketConfig, RestConfig, StreamConfig};
pub use clients::r#trait::BinanceSpotClient;
pub use enums::BINANCE_ENUM_VERSION;
pub use errors::BINANCE_ERROR_VERSION;
pub use filters::BINANCE_FILTER_VERSION;

pub mod rest {
    pub use super::clients::rest::client;
}

pub mod websocket {
    pub use super::clients::websocket::client;
}

pub mod stream {
    pub use super::streams::client;
}