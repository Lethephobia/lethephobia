use crate::errors::IdError;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum SnapshotIdError {
    #[error("id error: {0}")]
    Id(#[from] IdError),
}
