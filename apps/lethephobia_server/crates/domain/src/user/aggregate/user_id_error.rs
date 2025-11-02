use uuid::Uuid;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum UserIdError {
    #[error("not uuid v7: {0}")]
    NotUuidV7(Uuid),
}
