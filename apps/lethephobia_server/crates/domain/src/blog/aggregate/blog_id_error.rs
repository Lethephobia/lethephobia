use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum BlogIdError {
    #[error("not uuid v7: {0}")]
    NotUuidV7(Uuid),
}
