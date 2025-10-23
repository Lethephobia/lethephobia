use appletheia::event::Event;

use super::BlogEventPayload;
use crate::blog::aggregate::BlogId;

pub type BlogEvent = Event<BlogId, BlogEventPayload>;
