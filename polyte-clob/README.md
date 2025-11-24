# polyte-clob

Rust client library for Polymarket CLOB (Central Limit Order Book) API.

The CLOB API enables trading operations on Polymarket, including order creation, signing, posting, and account management.

More information about this crate can be found in the [crate documentation](https://docs.rs/polyte-clob/).

## Features

- **EIP-712 Order Signing**: Ethereum-standard order signing for security
- **Complete Trading Flow**: Create, sign, and post orders with a single method
- **Account Management**: Check balances, allowances, and trade history
- **Order Management**: List and cancel orders
- **Market Data**: Get order books, prices, and market information

## Installation

```toml
[dependencies]
polyte-clob = "0.1.0"
tokio = { version = "1", features = ["full"] }
```

Or use the unified client:

```toml
[dependencies]
polyte = "0.1.0"
```

## Usage

### Setup

```rust
use polyte_clob::{Clob, Credentials, Chain};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load credentials (from environment or secure storage)
    let private_key = "0x..."; // Your Ethereum private key
    let credentials = Credentials {
        key: "api_key".to_string(),
        secret: "api_secret".to_string(),
        passphrase: "passphrase".to_string(),
    };

    // Create CLOB client
    let clob = Clob::builder(private_key, credentials)?
        .chain(Chain::PolygonMainnet)
        .build()?;

    Ok(())
}
```

### Place an Order

```rust
use polyte_clob::{CreateOrderParams, OrderSide};

let params = CreateOrderParams {
    token_id: "token_id_here".to_string(),
    price: 0.52,
    size: 100.0,
    side: OrderSide::Buy,
    expiration: None,
};

// Single method to create, sign, and post order
let response = clob.place_order(&params).await?;

if response.success {
    println!("Order placed: {:?}", response.order_id);
} else {
    eprintln!("Order failed: {:?}", response.error_msg);
}
```

### Advanced: Manual Order Flow

```rust
// 1. Create unsigned order
let order = clob.create_order(&params).await?;

// 2. Sign the order with EIP-712
let signed_order = clob.sign_order(&order).await?;

// 3. Post to the order book
let response = clob.post_order(&signed_order).await?;
```

### Check Balance and Allowance

```rust
let balance = clob.account()
    .balance_allowance()
    .send()
    .await?;

println!("Balance: {}", balance.balance);
println!("Allowance: {}", balance.allowance);
```

### List Your Orders

```rust
let orders = clob.orders().list().send().await?;

for order in orders {
    println!("Order {}: {} @ {}",
        order.id,
        order.order.side,
        order.order.maker_amount
    );
}
```

### Cancel an Order

```rust
let response = clob.orders()
    .cancel("order_id_here")
    .send()
    .await?;

if response.success {
    println!("Order cancelled");
}
```

### Get Order Book

```rust
let order_book = clob.markets()
    .order_book("token_id_here")
    .send()
    .await?;

println!("Bids: {} levels", order_book.bids.len());
println!("Asks: {} levels", order_book.asks.len());
```

### Get Market Price

```rust
use polyte_clob::OrderSide;

let price = clob.markets()
    .price("token_id_here", OrderSide::Buy)
    .send()
    .await?;

println!("Buy price: {}", price.price);
```

## Configuration

```rust
let clob = Clob::builder(private_key, credentials)?
    .base_url("https://clob.polymarket.com")
    .timeout_ms(30_000)
    .pool_size(10)
    .chain(Chain::PolygonMainnet) // or Chain::PolygonAmoy for testnet
    .build()?;
```

## Supported Chains

- **Polygon Mainnet** (chain ID: 137) - Production
- **Polygon Amoy** (chain ID: 80002) - Testnet

## Authentication

The CLOB API requires two types of authentication:

1. **Private Key**: For signing orders with EIP-712 (on-chain security)
2. **API Credentials**: For authenticating API requests (off-chain)
   - API Key
   - API Secret
   - Passphrase

### Obtaining API Credentials

Visit the [Polymarket API documentation](https://docs.polymarket.com) for instructions on obtaining API credentials.

## Security Notes

- **Never commit private keys or API credentials** to version control
- Store credentials securely (environment variables, secrets manager)
- Use testnet (Polygon Amoy) for development and testing
- Validate all order parameters before signing

## Examples

The crate includes examples:

```bash
# Check balance and allowance
cargo run --example balance_allowance

# Retrieve market data
cargo run --example retrieve_markets
```

## API Coverage

- **Orders**: Create, sign, post, list, cancel
- **Markets**: Get market info, order book, prices, midpoint
- **Account**: Balance, allowance, trade history

## License

This project is licensed under the [MIT License](https://github.com/roushou/polyte/blob/main/LICENSE).
