//! # polyte-core
//!
//! Core utilities and shared types for Polyte Polymarket API clients.
//!
//! This crate provides common functionality used across `polyte-clob`, `polyte-gamma`, and `polyte-data`:
//! - Shared error types and error handling
//! - HTTP client configuration
//! - Request builder utilities
//!
//! ## HTTP Client
//!
//! Use [`HttpClientBuilder`] to create configured HTTP clients:
//!
//! ```
//! use polyte_core::HttpClientBuilder;
//!
//! let client = HttpClientBuilder::new("https://api.example.com")
//!     .timeout_ms(60_000)
//!     .build()
//!     .unwrap();
//! ```
//!
//! ## Error Handling
//!
//! Use the [`impl_api_error_conversions`] macro to reduce boilerplate in error types.

pub mod client;
pub mod error;
pub mod request;

pub use client::{HttpClient, HttpClientBuilder, DEFAULT_POOL_SIZE, DEFAULT_TIMEOUT_MS};
pub use error::ApiError;
pub use request::{QueryBuilder, Request, RequestError};
