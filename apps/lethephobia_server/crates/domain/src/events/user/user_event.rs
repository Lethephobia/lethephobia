use crate::events::{Event, UserEventPayload};
use crate::value_objects::UserId;

pub type UserEvent = Event<UserId, UserEventPayload>;
