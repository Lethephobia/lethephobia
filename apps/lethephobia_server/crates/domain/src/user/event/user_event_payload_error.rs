use thiserror::Error;

#[derive(Debug, Error)]
pub enum UserEventPayloadError {
    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),
}
