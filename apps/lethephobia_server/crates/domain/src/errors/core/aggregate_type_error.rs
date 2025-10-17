use thiserror::Error;

#[derive(Debug, Error)]
pub enum AggregateTypeError {
    #[error("failed to parse aggregate type: {0}")]
    FailedToParse(String),
}
