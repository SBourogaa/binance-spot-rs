use std::collections::HashMap;

use anyhow::Context;
use tokio::sync::{broadcast, mpsc, oneshot};
use tracing::{info, debug, instrument};

use crate::Result;
use crate::{
    BinanceConfig, 
    StreamConfig
};
use crate::config::{StreamMode, StreamType};
use super::specs::StreamSpec;
use super::connection::{
    MarketDataConnectionManager, 
    UserDataConnectionManager,
    ConnectionManager,
    StreamMessage, 
    ConnectionStatus, 
    ValueSender, ValueReceiver
};

type TypedReceiver<T> = broadcast::Receiver<T>;

/**
 * Typed subscription handle for WebSocket streams.
 *
 * Provides a typed interface for receiving stream events of a specific type.
 * Automatically handles deserialization and cleanup when dropped.
 *
 * # Type Parameters
 * - `T`: The event type for this subscription.
 *
 * # Fields
 * - `receiver`: Broadcast receiver for typed events.
 * - `task_handle`: Background task handle for automatic cleanup.
 */
pub struct TypedSubscription<T> {
    receiver: TypedReceiver<T>,
    task_handle: tokio::task::JoinHandle<()>,
}

impl<T: Clone> TypedSubscription<T> {
    /**
     * Receives the next event from the stream.
     *
     * # Returns
     * - Result containing the next event or receive error.
     */
    pub async fn recv(&mut self) -> std::result::Result<T, broadcast::error::RecvError> {
        self.receiver.recv().await
    }
    
    /**
     * Gets a mutable reference to the underlying receiver.
     *
     * # Returns
     * - Mutable reference to the broadcast receiver.
     */
    pub fn receiver(&mut self) -> &mut TypedReceiver<T> {
        &mut self.receiver
    }
}

impl<T> Drop for TypedSubscription<T> {
    fn drop(&mut self) {
        self.task_handle.abort();
    }
}

/**
 * Client operation mode.
 *
 * Determines how the client handles subscriptions and message routing.
 *
 * # Variants
 * - `Dynamic`: Supports runtime subscription management.
 * - `Static`: Uses pre-configured streams with fixed routing.
 */
enum ClientMode {
    /**
     * Dynamic mode with runtime subscription management.
     *
     * # Fields
     * - `sender`: Channel for sending subscription commands.
     */
    Dynamic {
        sender: mpsc::UnboundedSender<StreamMessage>,
    },
    
    /**
     * Static mode with pre-configured streams.
     *
     * # Fields
     * - `sender`: Channel for sending commands (limited functionality).
     * - `senders`: Map of stream names to broadcast senders.
     */
    Static {
        sender: mpsc::UnboundedSender<StreamMessage>,
        senders: HashMap<String, ValueSender>,
    },
}


/**
 * Market data stream client type.
 */
pub type MarketDataStreamClient = StreamClient<MarketDataConnectionManager>;

/**
 * User data stream client type.
 */
pub type UserDataStreamClient = StreamClient<UserDataConnectionManager>;

/**
 * Internal client implementation - not part of public API.
 *
 * Provides the core functionality for WebSocket stream management.
 *
 * # Type Parameters
 * - `M`: Connection manager type that implements ConnectionManager trait.
 *
 * # Fields
 * - `connection_manager`: Manages WebSocket connection lifecycle.
 * - `mode`: Client operation mode with associated resources.
 */
pub struct StreamClient<M: ConnectionManager> {
    connection_manager: M,
    mode: ClientMode,
}

impl<M: ConnectionManager> StreamClient<M> {
    /**
     * Creates a new stream client with the given connection manager and mode.
     *
     * # Arguments
     * - `connection_manager`: Connection manager instance.
     * - `mode`: Client operation mode.
     *
     * # Returns
     * - New StreamClient instance.
     */
    fn new_with_manager(connection_manager: M, mode: ClientMode) -> Self {
        Self {
            connection_manager,
            mode,
        }
    }
}

impl StreamClient<MarketDataConnectionManager> {
    /**
     * Creates a new market data stream client based on configuration.
     *
     * # Arguments
     * - `config`: Binance configuration with stream settings.
     *
     * # Returns
     * - New market data stream client.
     */
    pub(crate) fn new_market_data(config: BinanceConfig<StreamConfig>) -> Result<Self> {
        match config.stream_config().stream_mode() {
            StreamMode::Dynamic => {
                let (connection_manager, message_sender) = MarketDataConnectionManager::new_dynamic(config)?;
                let mode = ClientMode::Dynamic { sender: message_sender };
                Ok(Self::new_with_manager(connection_manager, mode))
            }
            StreamMode::Raw(_) | StreamMode::Combined(_) => {
                let (connection_manager, message_sender, senders) = MarketDataConnectionManager::new_static(config)?;
                let mode = ClientMode::Static { sender: message_sender, senders };
                Ok(Self::new_with_manager(connection_manager, mode))
            }
        }
    }
}

impl StreamClient<UserDataConnectionManager> {
    /**
     * Creates a new user data stream client.
     *
     * # Arguments
     * - `config`: Binance configuration with authentication credentials.
     *
     * # Returns
     * - New user data stream client.
     */
    pub(crate) fn new_user_data(config: BinanceConfig<StreamConfig>) -> Result<Self> {
        let (connection_manager, message_sender) = UserDataConnectionManager::new(config)?;
        let mode = ClientMode::Dynamic { sender: message_sender };
        Ok(Self::new_with_manager(connection_manager, mode))
    }
}

/**
 * Factory for creating appropriate stream clients based on configuration.
 */
pub(crate) struct StreamClientFactory;

impl StreamClientFactory {
    /**
     * Creates the appropriate stream client based on stream specification.
     *
     * # Arguments
     * - `config`: Binance configuration with stream settings.
     * - `_spec`: Stream specification (unused, for API compatibility).
     *
     * # Returns
     * - New BinanceSpotStreamClient instance.
     */
    pub fn for_stream<S: StreamSpec>(config: BinanceConfig<StreamConfig>, _spec: &S) -> Result<BinanceSpotStreamClient> {
        Self::new(config)
    }

    /**
     * Creates a new stream client based on the configuration.
     *
     * # Arguments
     * - `config`: Binance configuration with stream settings and type.
     *
     * # Returns
     * - New BinanceSpotStreamClient instance.
     */
    pub fn new(config: BinanceConfig<StreamConfig>) -> Result<BinanceSpotStreamClient> {
        match config.stream_config().stream_type {
            StreamType::MarketData => {
                let client = StreamClient::new_market_data(config)?;
                Ok(BinanceSpotStreamClient::MarketData(client))
            }
            StreamType::UserData => {
                let client = StreamClient::new_user_data(config)?;
                Ok(BinanceSpotStreamClient::UserData(client))
            }
        }
    }
}

impl<M: ConnectionManager> StreamClient<M> {
    /**
     * Waits for the WebSocket connection to be established.
     *
     * # Returns
     * - `()` on successful connection.
     */
    pub(crate) async fn wait_for_connection(&mut self) -> Result<()> {
        self.connection_manager.wait_for_connection().await
    }

    /**
     * Subscribes to a WebSocket stream.
     *
     * # Arguments
     * - `spec`: Stream specification defining the subscription.
     *
     * # Returns
     * - TypedSubscription for receiving stream events.
     */
    #[instrument(skip(self, spec), fields(stream_name = spec.stream_name()))]
    pub(crate) async fn subscribe<S: StreamSpec>(&mut self, spec: &S) -> Result<TypedSubscription<S::Event>> 
    where 
        S::Event: serde::de::DeserializeOwned + Clone + Send + 'static
    {
        let start = std::time::Instant::now();
        spec.validate()?;
        let buffer_size = spec.buffer_size(self.connection_manager.stream_config());
        let stream_name = spec.stream_name();
        
        let raw_receiver = match &self.mode {
            ClientMode::Dynamic { sender } => {
                let (tx, rx) = broadcast::channel(buffer_size);
                let (response_tx, response_rx) = oneshot::channel();
                
                sender.send(StreamMessage::Subscribe {
                    stream_name: stream_name.clone(),
                    sender: tx,
                    response: response_tx,
                }).context("Failed to send subscribe message")?;
                
                response_rx.await
                    .context("Failed to receive subscribe response")??;
                
                rx
            }
            ClientMode::Static { senders, .. } => {
                senders.get(&stream_name)
                    .map(|sender| sender.subscribe())
                    .ok_or_else(|| anyhow::anyhow!("Stream '{}' not configured in static mode", stream_name))?
            }
        };
        
        let subscription = self.create_typed_subscription::<S>(raw_receiver, buffer_size);
        
        info!(
            stream = %stream_name,
            buffer_size = buffer_size,
            duration_us = start.elapsed().as_micros(),
            "Stream subscription completed"
        );
        
        Ok(subscription)
    }

    /**
     * Unsubscribes from a WebSocket stream.
     *
     * # Arguments
     * - `spec`: Stream specification to unsubscribe from.
     *
     * # Returns
     * - `()` on successful unsubscription.
     */
    #[instrument(skip(self, spec), fields(stream_name = spec.stream_name()))]
    pub(crate) async fn unsubscribe<S: StreamSpec>(&self, spec: S) -> Result<()> {
        let _start = std::time::Instant::now();
        match &self.mode {
            ClientMode::Dynamic { sender } => {
                let (response_tx, response_rx) = oneshot::channel();
                
                sender.send(StreamMessage::Unsubscribe {
                    stream_names: vec![spec.stream_name()],
                    response: response_tx,
                }).context("Failed to send unsubscribe message")?;
                
                response_rx.await
                    .context("Failed to receive unsubscribe response")??;
                
                Ok(())
            }
            ClientMode::Static { .. } => {
                Err(anyhow::anyhow!("Unsubscribe not supported in static mode"))
            }
        }
    }

    /**
     * Gracefully closes the client connection.
     *
     * # Returns
     * - `()` on successful closure.
     */
    #[instrument(skip(self))]
    pub(crate) async fn close(&mut self) -> Result<()> {
        let start = std::time::Instant::now();
        let (response_tx, response_rx) = oneshot::channel();
        
        let sender = match &self.mode {
            ClientMode::Dynamic { sender } => sender,
            ClientMode::Static { sender, .. } => sender,
        };
        
        let _ = sender.send(StreamMessage::Shutdown(response_tx));
        
        match tokio::time::timeout(std::time::Duration::from_secs(10), response_rx).await {
            Ok(Ok(result)) => result?,
            Ok(Err(_)) | Err(_) => {},
        }

        self.connection_manager.abort_connection();
        
        info!(
            duration_us = start.elapsed().as_micros(),
            "Stream client closed"
        );
        Ok(())
    }

    /**
     * Creates a typed subscription from a raw receiver.
     *
     * # Arguments
     * - `raw_receiver`: Raw JSON value receiver.
     * - `buffer_size`: Buffer size for the typed channel.
     *
     * # Returns
     * - TypedSubscription for the specified event type.
     */
    fn create_typed_subscription<S: StreamSpec>(
        &self,
        mut raw_receiver: ValueReceiver,
        buffer_size: usize,
    ) -> TypedSubscription<S::Event> 
    where 
        S::Event: serde::de::DeserializeOwned + Clone + Send + 'static
    {
        let (typed_sender, typed_receiver) = broadcast::channel(buffer_size);
        
        let task_handle = tokio::spawn(async move {
            let mut message_count = 0u64;
            let mut parse_errors = 0u64;
            let last_stats_time = std::time::Instant::now();
            
            while let Ok(value) = raw_receiver.recv().await {
                message_count += 1;
                
                match serde_json::from_value::<S::Event>(value.clone()) {
                    Ok(typed_event) => {
                        if typed_sender.send(typed_event).is_err() {
                            debug!(
                                messages_processed = message_count,
                                parse_errors = parse_errors,
                                "Typed subscription channel closed, terminating"
                            );
                            break;
                        }
                    }
                    Err(_) => {
                        parse_errors += 1;
                        continue;
                    }
                }
                
                if message_count % 1000 == 0 {
                    let rate = message_count as f64 / last_stats_time.elapsed().as_secs_f64();
                    debug!(
                        messages_processed = message_count,
                        parse_errors = parse_errors,
                        messages_per_second = rate,
                        "Typed subscription processing stats"
                    );
                }
            }
        });
        
        TypedSubscription {
            receiver: typed_receiver,
            task_handle,
        }
    }


    pub(crate) fn connection_status(&self) -> ConnectionStatus {
        self.connection_manager.connection_status()
    }

    pub(crate) fn is_connected(&self) -> bool {
        self.connection_manager.is_connected()
    }

    pub(crate) fn market_data_url(&self) -> &str {
        self.connection_manager.market_data_url()
    }

    pub(crate) fn user_data_url(&self) -> &str {
        self.connection_manager.user_data_url()
    }

    pub(crate) fn has_authentication(&self) -> bool {
        self.connection_manager.has_authentication()
    }

    pub(crate) fn stream_config(&self) -> &StreamConfig {
        self.connection_manager.stream_config()
    }
}

impl<M: ConnectionManager> Drop for StreamClient<M> {
    fn drop(&mut self) {
        self.connection_manager.abort_connection();
    }
}

/**
 * Public API for Binance WebSocket streams.
 *
 * Provides a clean, typed interface for subscribing to Binance WebSocket
 * streams. Supports both dynamic subscription management and static
 * pre-configured stream connections.
 *
 * # Variants
 * - `MarketData`: Market data stream client for public data.
 * - `UserData`: User data stream client for private account data.
 */
pub enum BinanceSpotStreamClient {
    MarketData(MarketDataStreamClient),
    UserData(UserDataStreamClient),
}

impl BinanceSpotStreamClient {
    /**
     * Creates a new Binance spot stream client with automatic stream type detection.
     *
     * # Arguments
     * - `config`: Binance configuration with stream settings.
     * - `spec`: Stream specification to determine client requirements.
     *
     * # Returns
     * - New BinanceSpotStreamClient instance.
     */
    pub fn for_stream<S: StreamSpec>(config: BinanceConfig<StreamConfig>, spec: &S) -> Result<Self> {
        StreamClientFactory::for_stream(config, spec)
    }

    /**
     * Creates a new Binance spot stream client based on configuration.
     *
     * # Arguments
     * - `config`: Binance configuration with stream settings and type.
     *
     * # Returns
     * - New BinanceSpotStreamClient instance.
     */
    pub fn new(config: BinanceConfig<StreamConfig>) -> Result<Self> {
        StreamClientFactory::new(config)
    }

    /**
     * Waits for the WebSocket connection to be established.
     *
     * # Returns
     * - `()` on successful connection.
     */
    pub async fn wait_for_connection(&mut self) -> Result<()> {
        match self {
            BinanceSpotStreamClient::MarketData(client) => client.wait_for_connection().await,
            BinanceSpotStreamClient::UserData(client) => client.wait_for_connection().await,
        }
    }

    /**
     * Subscribes to a WebSocket stream.
     *
     * # Arguments
     * - `spec`: Stream specification defining the subscription.
     *
     * # Returns
     * - TypedSubscription for receiving stream events.
     */
    pub async fn subscribe<S: StreamSpec>(&mut self, spec: &S) -> Result<TypedSubscription<S::Event>> 
    where 
        S::Event: serde::de::DeserializeOwned + Clone + Send + 'static
    {
        match self {
            BinanceSpotStreamClient::MarketData(client) => client.subscribe(spec).await,
            BinanceSpotStreamClient::UserData(client) => client.subscribe(spec).await,
        }
    }

    /**
     * Unsubscribes from a WebSocket stream.
     *
     * # Arguments
     * - `spec`: Stream specification to unsubscribe from.
     *
     * # Returns
     * - `()` on successful unsubscription.
     */
    pub async fn unsubscribe<S: StreamSpec>(&mut self, spec: S) -> Result<()> {
        match self {
            BinanceSpotStreamClient::MarketData(client) => client.unsubscribe(spec).await,
            BinanceSpotStreamClient::UserData(client) => client.unsubscribe(spec).await,
        }
    }

    /**
     * Gracefully closes the client connection.
     *
     * # Returns
     * - `()` on successful closure.
     */
    pub async fn close(&mut self) -> Result<()> {
        match self {
            BinanceSpotStreamClient::MarketData(client) => client.close().await,
            BinanceSpotStreamClient::UserData(client) => client.close().await,
        }
    }

    pub fn connection_status(&self) -> ConnectionStatus {
        match self {
            BinanceSpotStreamClient::MarketData(client) => client.connection_status(),
            BinanceSpotStreamClient::UserData(client) => client.connection_status(),
        }
    }

    pub fn is_connected(&self) -> bool {
        match self {
            BinanceSpotStreamClient::MarketData(client) => client.is_connected(),
            BinanceSpotStreamClient::UserData(client) => client.is_connected(),
        }
    }

    pub fn market_data_url(&self) -> &str {
        match self {
            BinanceSpotStreamClient::MarketData(client) => client.market_data_url(),
            BinanceSpotStreamClient::UserData(client) => client.market_data_url(),
        }
    }

    pub fn user_data_url(&self) -> &str {
        match self {
            BinanceSpotStreamClient::MarketData(client) => client.user_data_url(),
            BinanceSpotStreamClient::UserData(client) => client.user_data_url(),
        }
    }

    pub fn has_authentication(&self) -> bool {
        match self {
            BinanceSpotStreamClient::MarketData(client) => client.has_authentication(),
            BinanceSpotStreamClient::UserData(client) => client.has_authentication(),
        }
    }

    pub fn stream_config(&self) -> &StreamConfig {
        match self {
            BinanceSpotStreamClient::MarketData(client) => client.stream_config(),
            BinanceSpotStreamClient::UserData(client) => client.stream_config(),
        }
    }
}