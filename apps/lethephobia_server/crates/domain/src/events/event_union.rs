use crate::events::{BlogEvent, UserEvent};

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum EventUnion {
    User(UserEvent),
    Blog(BlogEvent),
}

impl EventUnion {
    pub fn aggregate_type(&self) -> &'static str {
        match self {
            Self::User(_) => "user",
            Self::Blog(_) => "blog",
        }
    }
}
