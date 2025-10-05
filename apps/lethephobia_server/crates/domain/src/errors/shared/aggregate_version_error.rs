use thiserror::Error;

use crate::errors::DomainError;

#[derive(Debug, Error)]
pub enum AggregateVersionError {
    #[error("aggregate version must be positive, got {0}")]
    NegativeValue(i64),
    #[error("aggregate version overflow")]
    Overflow,
}

impl DomainError for AggregateVersionError {}
