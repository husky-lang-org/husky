use disk_cache::error::DiskCacheError;
use thiserror::Error;
use usage_cap::error::UsageCapError;

#[derive(Debug, Error)]
pub enum GeminiError {
    #[error("DiskCache error: {0}")]
    DiskCache(#[from] DiskCacheError),
    #[error("UsageCap error: {0}")]
    UsageCap(#[from] UsageCapError),
    #[error("Request failed: {0}")]
    RequestFailed(#[from] reqwest::Error),
    #[error("Failed to parse response: {error}. Response body: {response_text}")]
    ResponseParseFailed {
        error: serde_json::Error,
        response_text: String,
    },
    #[error("API error: {status} ({code}): {message}")]
    ApiError {
        code: i32,
        message: String,
        status: String,
    },
}

pub type GeminiResult<T> = Result<T, GeminiError>;
