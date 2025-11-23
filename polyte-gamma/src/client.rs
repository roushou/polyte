use std::time::Duration;

use reqwest::Client;
use url::Url;

use crate::{
    api::{
        comments::Comments, events::Events, markets::Markets, series::Series, sports::Sports,
        tags::Tags,
    },
    error::Result,
};

const DEFAULT_BASE_URL: &str = "https://gamma-api.polymarket.com";
const DEFAULT_TIMEOUT_MS: u64 = 30_000;
const DEFAULT_POOL_SIZE: usize = 10;

/// Main Gamma API client
#[derive(Clone)]
pub struct Gamma {
    pub(crate) client: Client,
    pub(crate) base_url: Url,
}

impl Gamma {
    /// Create a new Gamma client with default configuration
    pub fn new() -> Result<Self> {
        Self::builder().build()
    }

    /// Create a builder for configuring the client
    pub fn builder() -> GammaBuilder {
        GammaBuilder::new()
    }

    /// Get markets namespace
    pub fn markets(&self) -> Markets {
        Markets {
            client: self.client.clone(),
            base_url: self.base_url.clone(),
        }
    }

    /// Get events namespace
    pub fn events(&self) -> Events {
        Events {
            client: self.client.clone(),
            base_url: self.base_url.clone(),
        }
    }

    /// Get series namespace
    pub fn series(&self) -> Series {
        Series {
            client: self.client.clone(),
            base_url: self.base_url.clone(),
        }
    }

    /// Get tags namespace
    pub fn tags(&self) -> Tags {
        Tags {
            client: self.client.clone(),
            base_url: self.base_url.clone(),
        }
    }

    /// Get sport namespace
    pub fn sport(&self) -> Sports {
        Sports {
            client: self.client.clone(),
            base_url: self.base_url.clone(),
        }
    }

    /// Get comments namespace
    pub fn comments(&self) -> Comments {
        Comments {
            client: self.client.clone(),
            base_url: self.base_url.clone(),
        }
    }
}

/// Builder for configuring Gamma client
pub struct GammaBuilder {
    base_url: String,
    timeout_ms: u64,
    pool_size: usize,
}

impl GammaBuilder {
    fn new() -> Self {
        Self {
            base_url: DEFAULT_BASE_URL.to_string(),
            timeout_ms: DEFAULT_TIMEOUT_MS,
            pool_size: DEFAULT_POOL_SIZE,
        }
    }

    /// Set base URL for the API
    pub fn base_url(mut self, url: impl Into<String>) -> Self {
        self.base_url = url.into();
        self
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

    /// Build the Gamma client
    pub fn build(self) -> Result<Gamma> {
        let client = Client::builder()
            .timeout(Duration::from_millis(self.timeout_ms))
            .pool_max_idle_per_host(self.pool_size)
            .build()?;

        let base_url = Url::parse(&self.base_url)?;

        Ok(Gamma { client, base_url })
    }
}

impl Default for GammaBuilder {
    fn default() -> Self {
        Self::new()
    }
}
