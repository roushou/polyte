//! # polyte-clob
//!
//! Rust client library for Polymarket CLOB (Centralized Limit Order Book) API.
//!
//! ## Features
//!
//! - Order creation, signing, and posting with EIP-712
//! - Market data and order book retrieval
//! - Account balance and trade history
//! - HMAC-based L2 authentication
//! - Type-safe API with idiomatic Rust patterns
//!
//! ## Example
//!
//! ```no_run
//! use polyte_clob::{Chain, Clob, CreateOrderParams, OrderSide, Wallet, Credentials};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let private_key = "0x..."; // Load this from environment variables
//!
//!     // Set up credentials (from API)
//!     let credentials = Credentials {
//!         key: "<api_key>".to_string(),
//!         secret: "<secret>".to_string(),
//!         passphrase: "<passphrase>".to_string(),
//!     };
//!
//!     // Create CLOB client
//!     let clob = Clob::builder(private_key, credentials)?
//!         .chain(Chain::PolygonMainnet)
//!         .build()?;
//!
//!     // Place an order
//!     let params = CreateOrderParams {
//!         token_id: "token_id".to_string(),
//!         price: 0.52,
//!         size: 100.0,
//!         side: OrderSide::Buy,
//!         expiration: None,
//!     };
//!
//!     let response = clob.place_order(&params).await?;
//!     println!("Order ID: {:?}", response.order_id);
//!
//!     Ok(())
//! }
//! ```

pub mod api;
pub mod client;
pub mod core;
pub mod error;
pub mod request;
pub mod signer;
pub mod types;
pub mod utils;
pub mod wallet;

pub use api::account::{BalanceAllowanceResponse, Trade};
pub use api::markets::{
    ListMarketsResponse, Market, MarketToken, MidpointResponse, OrderBook, OrderLevel,
    PriceResponse,
};
pub use api::orders::{CancelResponse, OpenOrder, OrderResponse};
pub use client::{Clob, ClobBuilder, CreateOrderParams};
pub use core::chain::{Chain, Contracts};
pub use error::{ClobError, Result};
pub use types::{Credentials, Order, OrderSide, SignatureType, SignedOrder, TickSize};
pub use wallet::Wallet;
