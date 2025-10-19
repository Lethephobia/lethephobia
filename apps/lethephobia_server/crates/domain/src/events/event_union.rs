use crate::events::{BlogEvent, UserEvent};
use crate::value_objects::AggregateType;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum EventUnion {
    User(UserEvent),
    Blog(BlogEvent),
}

impl EventUnion {
    pub fn aggregate_type(&self) -> AggregateType {
        match self {
            Self::User(_) => AggregateType::User,
            Self::Blog(_) => AggregateType::Blog,
        }
    }
}
