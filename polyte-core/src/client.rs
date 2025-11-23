use std::time::Duration;

use reqwest::Client;
use url::Url;

use crate::error::ApiError;

const DEFAULT_TIMEOUT_MS: u64 = 30_000;
const DEFAULT_POOL_SIZE: usize = 10;

/// Shared client configuration
#[derive(Debug, Clone)]
pub struct ClientConfig {
    pub client: Client,
    pub base_url: Url,
}

/// Builder for HTTP client configuration
pub struct ClientBuilder {
    base_url: String,
    timeout_ms: u64,
    pool_size: usize,
}

impl ClientBuilder {
    /// Create a new client builder
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into(),
            timeout_ms: DEFAULT_TIMEOUT_MS,
            pool_size: DEFAULT_POOL_SIZE,
        }
    }

    /// Set request timeout in milliseconds
    pub fn timeout_ms(mut self, timeout: u64) -> Self {
        self.timeout_ms = timeout;
        self
    }

    /// Set connection pool size
    pub fn pool_size(mut self, size: usize) -> Self {
        self.pool_size = size;
        self
    }

    /// Build the client configuration
    pub fn build(self) -> Result<ClientConfig, ApiError> {
        let client = Client::builder()
            .timeout(Duration::from_millis(self.timeout_ms))
            .pool_max_idle_per_host(self.pool_size)
            .build()?;

        let base_url = Url::parse(&self.base_url)?;

        Ok(ClientConfig { client, base_url })
    }
}
