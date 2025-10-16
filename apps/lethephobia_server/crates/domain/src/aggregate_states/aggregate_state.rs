use crate::value_objects::AggregateId;
use std::{fmt::Debug, hash::Hash};

pub trait AggregateState: Clone + Debug + Eq + Hash + Send + Sync + 'static {
    type Id: AggregateId;

    fn id(&self) -> Self::Id;
}
