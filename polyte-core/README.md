# polyte-core

Core utilities and shared types for Polyte Polymarket API clients.

This crate provides common functionality used by both `polyte-clob` and `polyte-gamma`:

- HTTP client configuration and building
- Shared error types
- Request building utilities
- Query parameter handling

More information about this crate can be found in the [crate documentation](https://docs.rs/polyte-core/).

## Usage

This crate is typically used as a dependency by other Polyte crates and not directly by end users. If you want to interact with Polymarket APIs, use the main [`polyte`](https://crates.io/crates/polyte) crate instead.

## Features

- **Client Building**: Configurable HTTP client with timeout and connection pooling
- **Error Handling**: Unified error types for API operations
- **Request Utilities**: Builder pattern for constructing API requests

## Installation

```toml
[dependencies]
polyte-core = "0.1.0"
```

## License

This project is licensed under the [MIT License](https://github.com/roushou/polyte/blob/main/LICENSE).
