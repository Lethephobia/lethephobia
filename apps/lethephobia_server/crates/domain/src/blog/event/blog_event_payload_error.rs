use thiserror::Error;

#[derive(Debug, Error)]
pub enum BlogEventPayloadError {
    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),
}
