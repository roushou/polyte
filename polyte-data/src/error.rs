use polyte_core::ApiError;
use thiserror::Error;

/// Result type for Data API operations
pub type Result<T> = std::result::Result<T, DataApiError>;

/// Error types for Data API operations
#[derive(Error, Debug)]
pub enum DataApiError {
    /// Core API error
    #[error(transparent)]
    Api(#[from] ApiError),
}

impl DataApiError {
    /// Create error from HTTP response
    pub(crate) async fn from_response(response: reqwest::Response) -> Self {
        Self::Api(ApiError::from_response(response).await)
    }
}

impl From<reqwest::Error> for DataApiError {
    fn from(err: reqwest::Error) -> Self {
        Self::Api(ApiError::Network(err))
    }
}

impl From<url::ParseError> for DataApiError {
    fn from(err: url::ParseError) -> Self {
        Self::Api(ApiError::Url(err))
    }
}

impl From<serde_json::Error> for DataApiError {
    fn from(err: serde_json::Error) -> Self {
        Self::Api(ApiError::Serialization(err))
    }
}
