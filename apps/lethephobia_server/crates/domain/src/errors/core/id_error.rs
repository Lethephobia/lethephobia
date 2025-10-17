use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum IdError {
    #[error("failed to parse id: {0}")]
    NotUuid(String),

    #[error("not a uuidv7: {0}")]
    NotUuidV7(Uuid),
}
