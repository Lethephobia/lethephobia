use std::{fmt::Debug, hash::Hash};

pub trait ValueObject: Debug + Eq + Hash + Send + Sync + 'static {}
