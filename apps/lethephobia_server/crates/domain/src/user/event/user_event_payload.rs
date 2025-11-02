use appletheia::event::EventPayload;
use serde::{Deserialize, Serialize};

use super::UserEventPayloadError;

#[derive(Clone, Debug, Eq, PartialEq, Hash, Deserialize, Serialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum UserEventPayload {}

impl EventPayload for UserEventPayload {
    type Error = UserEventPayloadError;
}
