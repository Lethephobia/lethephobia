use crate::errors::AggregateVersionError;
use std::error::Error;

pub trait AggregateError: Error + From<AggregateVersionError> + Send + Sync + 'static {}
