# Polyte

Rust client for Polymarket APIs.

More information about this crate can be found in the [crate documentation](https://docs.rs/polyte/).

## Crates

| Crate | Description |
|-------|-------------|
| [polyte](./) | Unified client for Polymarket APIs (CLOB, Gamma, Data, WebSocket) |
| [polyte-cli](../polyte-cli) | CLI tool for querying Polymarket APIs |
| [polyte-clob](../polyte-clob) | Client library for Polymarket CLOB (order book) API |
| [polyte-core](../polyte-core) | Core utilities and shared types |
| [polyte-data](../polyte-data) | Client library for Polymarket Data API |
| [polyte-gamma](../polyte-gamma) | Client library for Polymarket Gamma (market data) API |

## Installation

```
cargo add polyte
```

Or install individual APIs:

```
# Market data only
cargo add polyte --no-default-features --features gamma

# Trading only
cargo add polyte --no-default-features --features clob

# Data API only
cargo add polyte --no-default-features --features data

# WebSocket only
cargo add polyte --no-default-features --features ws
```

## Usage

### REST API

```rust
use polyte::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load account from environment variables
    let account = Account::from_env()?;

    let client = Polymarket::builder(account)
        .chain(Chain::PolygonMainnet)
        .build()?;

    // Get markets
    let markets = client.gamma.markets().list().send().await?;

    // Get balance
    let balance = client.clob.balance_allowance().await?;

    Ok(())
}
```

### WebSocket

```rust
use polyte::prelude::*;
use futures_util::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to market channel (no auth required)
    let mut ws = ws::WebSocket::connect_market(vec![
        "token_id".to_string(),
    ]).await?;

    while let Some(msg) = ws.next().await {
        match msg? {
            ws::Channel::Market(ws::MarketMessage::Book(book)) => {
                println!("Order book: {} bids, {} asks", book.bids.len(), book.asks.len());
            }
            ws::Channel::Market(ws::MarketMessage::PriceChange(pc)) => {
                println!("Price change: {:?}", pc.price_changes);
            }
            _ => {}
        }
    }

    Ok(())
}
```

## License

This project is licensed under the [MIT](./LICENSE) License.
