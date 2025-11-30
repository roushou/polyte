use std::marker::PhantomData;

pub use polyte_core::request::QueryBuilder;
use reqwest::{Client, Response};
use serde::de::DeserializeOwned;
use url::Url;

use crate::error::{DataApiError, Result};

/// Generic request builder for Data API
pub struct Request<T> {
    pub(crate) client: Client,
    pub(crate) base_url: Url,
    pub(crate) path: String,
    pub(crate) query: Vec<(String, String)>,
    pub(crate) _marker: PhantomData<T>,
}

impl<T> Request<T> {
    /// Create a new request
    pub(crate) fn new(client: Client, base_url: Url, path: String) -> Self {
        Self {
            client,
            base_url,
            path,
            query: Vec::new(),
            _marker: PhantomData,
        }
    }
}

impl<T> QueryBuilder for Request<T> {
    fn add_query(&mut self, key: String, value: String) {
        self.query.push((key, value));
    }
}

impl<T: DeserializeOwned> Request<T> {
    /// Execute the request and deserialize response
    pub async fn send(self) -> Result<T> {
        let url = self.base_url.join(&self.path)?;

        let mut request = self.client.get(url);

        if !self.query.is_empty() {
            request = request.query(&self.query);
        }

        tracing::debug!("Sending request to: {:?}", request);

        let response = request.send().await?;
        let status = response.status();

        tracing::debug!("Response status: {}", status);

        if !status.is_success() {
            let error = DataApiError::from_response(response).await;
            tracing::error!("Request failed: {:?}", error);
            return Err(error);
        }

        // Get text for debugging
        let text = response.text().await?;

        tracing::debug!("Response body: {}", text);

        // Deserialize and provide better error context
        serde_json::from_str(&text).map_err(|e| {
            tracing::error!("Deserialization failed: {}", e);
            tracing::error!("Failed to deserialize: {}", text);
            e.into()
        })
    }

    /// Execute the request and return raw response
    pub async fn send_raw(self) -> Result<Response> {
        let url = self.base_url.join(&self.path)?;

        let mut request = self.client.get(url);

        if !self.query.is_empty() {
            request = request.query(&self.query);
        }

        tracing::debug!("Sending request to: {:?}", request);

        let response = request.send().await?;
        let status = response.status();

        tracing::debug!("Response status: {}", status);

        if !status.is_success() {
            let error = DataApiError::from_response(response).await;
            tracing::error!("Request failed: {:?}", error);
            return Err(error);
        }

        Ok(response)
    }
}
