use std::error::Error;

pub trait DomainError: Error + Send + Sync + 'static {}
