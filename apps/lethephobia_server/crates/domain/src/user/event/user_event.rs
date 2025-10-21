use appletheia::event::Event;

use super::UserEventPayload;
use crate::user::aggregate::UserId;

pub type UserEvent = Event<UserId, UserEventPayload>;
