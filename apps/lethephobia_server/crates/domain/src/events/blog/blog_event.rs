use crate::events::{BlogEventPayload, Event};
use crate::value_objects::BlogId;

pub type BlogEvent = Event<BlogId, BlogEventPayload>;
