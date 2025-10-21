use appletheia::identifier::IdError;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum BlogIdError {
    #[error("id error: {0}")]
    Id(#[from] IdError),
}
