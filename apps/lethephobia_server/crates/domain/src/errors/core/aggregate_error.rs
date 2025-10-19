use thiserror::Error;

use super::aggregate_version_error::AggregateVersionError;
use crate::value_objects::{AggregateId, AggregateVersion};

#[derive(Debug, Error)]
pub enum AggregateError<A: AggregateId> {
    #[error("invalid aggregate id: {0}, expected {1}")]
    InvalidAggregateId(A, A),

    #[error("aggregate version error: {0}")]
    Version(#[from] AggregateVersionError),

    #[error("invalid next event version: {0}, expected {1}")]
    InvalidNextEventVersion(AggregateVersion, AggregateVersion),

    #[error("no state")]
    NoState,
}
