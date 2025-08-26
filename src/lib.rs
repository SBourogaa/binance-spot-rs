mod auth;
pub mod clients;
mod config;
pub mod enums;
mod errors;
mod filters;
pub mod streams;
pub mod types;

pub use auth::Ed25519Signer;
pub use clients::r#trait::BinanceSpotClient;
pub use config::{BinanceConfig, RestConfig, StreamConfig, WebSocketConfig};
pub use enums::BINANCE_ENUM_VERSION;
pub use errors::BINANCE_ERROR_VERSION;
pub use errors::BinanceError;
pub use filters::BINANCE_FILTER_VERSION;

pub type Result<T> = anyhow::Result<T>;

pub mod rest {
    pub use super::clients::rest::client;
}

pub mod websocket {
    pub use super::clients::websocket::client;
}

pub mod stream {
    pub use super::streams::client;
}
