use crate::{BinanceConfig, StreamConfig};

/**
 * Stream endpoint configuration.
 *
 * Represents different types of WebSocket endpoint configurations
 * based on the stream mode specified in the configuration.
 *
 * # Variants
 * - `Single`: Connect to a single specific stream.
 * - `Combined`: Connect to multiple pre-defined streams.
 * - `Dynamic`: Connect to the dynamic subscription endpoint.
 */
pub(super) enum StreamEndpoint {
    Single(String),
    Combined(Vec<String>),
    Dynamic,
}

impl StreamEndpoint {
    /**
     * Creates a stream endpoint from configuration.
     *
     * # Arguments
     * - `config`: Binance configuration containing stream mode settings.
     *
     * # Returns
     * - StreamEndpoint instance matching the configuration.
     */
    pub fn from_config(config: &BinanceConfig<StreamConfig>) -> Self {
        match config.stream_config().stream_mode() {
            crate::config::StreamMode::Raw(stream_info) => {
                Self::Single(stream_info.name.clone())
            }
            crate::config::StreamMode::Combined(stream_infos) => {
                let streams = stream_infos.iter().map(|info| info.name.clone()).collect();
                Self::Combined(streams)
            }
            crate::config::StreamMode::Dynamic => {
                Self::Dynamic
            }
        }
    }

    /**
     * Builds the WebSocket URL for this endpoint.
     *
     * # Arguments
     * - `base_url`: Base URL for the WebSocket endpoint.
     *
     * # Returns
     * - Complete WebSocket URL string.
     */
    pub fn build_url(&self, base_url: &str) -> String {
        let base = base_url.trim_end_matches('/');
        
        match self {
            Self::Single(stream) => {
                format!("{}/ws/{}", base, stream)
            }
            Self::Combined(streams) => {
                let streams_param = streams.join("/").replace("+", "%2B");
                format!("{}/stream?streams={}", base, streams_param)
            }
            Self::Dynamic => {
                format!("{}/stream", base)
            }
        }
    }
}