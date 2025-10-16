use crate::errors::IdError;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum EventIdError {
    #[error("id error: {0}")]
    Id(#[from] IdError),
}
