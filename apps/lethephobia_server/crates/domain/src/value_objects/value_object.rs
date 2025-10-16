use std::{fmt::Debug, hash::Hash};

pub trait ValueObject: Clone + Debug + Eq + Hash + Send + Sync + 'static {}
