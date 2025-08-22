# Binance Spot API Client

A Rust library for interacting with the Binance Spot API, providing REST/WebSocket API access and WebSocket Streams support.

## Quick Start

### REST API

```rust
use binance_spot_rs::{
    BinanceConfig, RestConfig, rest,
    clients::r#trait::{GeneralClient, MarketDataClient},
    types::requests::OrderBookSpec,
    Result,
};

#[tokio::main]
async fn main() -> Result<()> {
    let config = BinanceConfig::<RestConfig>::builder()
        .with_testnet()
        .build()?;
    
    let client = rest::client(config)?;
    
    client.ping().await?;
    
    let server_time = client.server_time().await?;
    println!("Server time: {}", server_time.server_time);
    
    let order_book_spec = OrderBookSpec::new("BTCUSDT")
        .with_limit(5)
        .build()?;
    let order_book = client.order_book(order_book_spec).await?;
    println!("BTCUSDT order book - {} bids, {} asks", 
        order_book.bids.len(), order_book.asks.len());
    
    Ok(())
}
```

### WebSocket API

```rust
use binance_spot_rs::{
    BinanceConfig, WebSocketConfig, websocket,
    clients::r#trait::{GeneralClient, MarketDataClient},
    types::requests::OrderBookSpec,
    Result,
};

#[tokio::main]
async fn main() -> Result<()> {
    let config = BinanceConfig::<WebSocketConfig>::builder()
        .with_testnet()
        .build()?;
    
    let mut client = websocket::client(config)?;
    
    client.wait_for_connection().await?;
    client.ping().await?;
    
    let server_time = client.server_time().await?;
    println!("Server time: {}", server_time.server_time);
    
    let order_book_spec = OrderBookSpec::new("BTCUSDT")
        .with_limit(5)
        .build()?;
    let order_book = client.order_book(order_book_spec).await?;
    println!("BTCUSDT order book - {} bids, {} asks", 
        order_book.bids.len(), order_book.asks.len());
    
    client.close().await?;
    
    Ok(())
}
```

### Streaming (Market Data)

```rust
use binance_spot_rs::{
    BinanceConfig, StreamConfig,
    streams::{BinanceSpotStreamClient, specs::TradeStreamSpec},
    Result,
};
use tokio::time::{timeout, Duration};

#[tokio::main] 
async fn main() -> Result<()> {
    let config = BinanceConfig::<StreamConfig>::builder()
        .with_testnet()
        .with_market_data()
        .build()?;
    
    let mut client = BinanceSpotStreamClient::new(config)?;
    
    client.wait_for_connection().await?;
    
    let trade_spec = TradeStreamSpec::new("BTCUSDT");
    let mut subscription = client.subscribe(&trade_spec).await?;
    
    let start_time = std::time::Instant::now();
    while start_time.elapsed() < Duration::from_secs(10) {
        match timeout(Duration::from_millis(500), subscription.recv()).await {
            Ok(Ok(trade)) => {
                println!("{} {} @ {} (ID: {})", 
                    trade.trade.quantity, 
                    trade.symbol, 
                    trade.trade.price, 
                    trade.trade.id
                );
            }
            Ok(Err(e)) => {
                eprintln!("Error: {}", e);
                break;
            }
            Err(_) => {
                // Timeout, continue listening
            }
        }
    }
    
    client.unsubscribe(trade_spec).await?;
    client.close().await?;
    
    Ok(())
}
```

### Streaming (User Data)

```rust
use binance_spot_rs::{
    BinanceConfig, StreamConfig,
    streams::{BinanceSpotStreamClient, specs::UserDataStreamSpec},
    Result,
};
use tokio::time::{timeout, Duration};

#[tokio::main]
async fn main() -> Result<()> {
    let api_key = std::env::var("BINANCE_API_KEY")
        .or_else(|_| std::env::var("BINANCE_TESTNET_API_KEY"))
        .expect("Please set BINANCE_API_KEY or BINANCE_TESTNET_API_KEY environment variable");
    
    let pem_file = std::env::var("BINANCE_PEM_FILE")
        .or_else(|_| std::env::var("BINANCE_TESTNET_PEM_FILE"))
        .expect("Please set BINANCE_PEM_FILE or BINANCE_TESTNET_PEM_FILE environment variable");
    
    let config = BinanceConfig::<StreamConfig>::builder()
        .with_testnet()
        .with_credentials_from_file(api_key, pem_file)?
        .with_user_data()
        .build()?;
    
    let mut client = BinanceSpotStreamClient::new(config)?;
    
    client.wait_for_connection().await?;
    
    let user_data_spec = UserDataStreamSpec::new();
    let mut subscription = client.subscribe(&user_data_spec).await?;
    
    match timeout(Duration::from_secs(30), subscription.recv()).await {
        Ok(Ok(event)) => {
            println!("User data event: {:#?}", event);
        }
        Ok(Err(e)) => {
            eprintln!("Error: {}", e);
        }
        Err(_) => {
            println!("No user data events received (normal if no account activity)");
        }
    }
    
    client.close().await?;
    
    Ok(())
}
```

## Authentication

For authenticated endpoints, provide API key and ED25519 private key file:

```rust
use binance_spot_rs::{BinanceConfig, RestConfig};

let config = BinanceConfig::<RestConfig>::builder()
    .with_testnet()
    .with_credentials_from_file("your_api_key", "path/to/ed25519_private_key.pem")?
    .build()?;
```

## Configuration Types

The library supports three main configuration types:

- `BinanceConfig<RestConfig>` - For REST API client
- `BinanceConfig<WebSocketConfig>` - For WebSocket API client  
- `BinanceConfig<StreamConfig>` - For Websocket Streams client

## REST/WebSocket API

The library supports **33 out of 47** endpoints for both REST and WebSocket API clients. The library is not supporting **deprecated endpoints and the following (yet)**:

- `place_oco_order()` - Place OCO orders
- `place_oto_order()` - Place OTO orders  
- `place_otoco_order()` - Place OTOCO orders
- `cancel_order_list()` - Cancel order lists
- `order_list_status()` - Query order lists
- `all_order_lists()` - Query all order lists
- `open_order_lists()` - Query open order lists
- `place_sor_order()` - Place SOR orders
- `test_sor_order()` - Test SOR orders
- `session_login()` - Session login (WebSocket only) - Used in the streaming client instead. 
- `session_status()` - Session status (WebSocket only) - Used in the streaming client instead.
- `session_logout()` - Session logout (WebSocket only) - Used in the streaming client instead.
- `subscribe_user_data()` - Subscribe to user data stream (WebSocket only) - Used in the streaming client instead.
- `unsubscribe_user_data()` - Unsubscribe from user data stream (WebSocket only) - Used in the streaming client instead.

## WebSocket Streams

- **Market Data Streams**: Full support. 
- **User Data Streams**: Full support. 

## Testing

Integration tests run against Binance TESTNET:

```bash
cargo test
```

Compatible with Binance API as of **August 22, 2025**.

---

**Disclaimer:**

> ⚠️ **NOT AFFILIATED WITH BINANCE.**

The software is provided "as is" without warranties or guarantees of any kind, express or implied. The authors are not responsible for any damages, including but not limited to financial losses, trading failures, or missed/duplicated orders resulting from its use. By using this software, you accept full responsibility for all outcomes.

> ⚠️ **CRYPTOCURRENCY AND TRADING INVOLVES SIGNIFICANT RISK, INCLUDING THE POSSIBILITY OF TOTAL LOSS OF CAPITAL.**
