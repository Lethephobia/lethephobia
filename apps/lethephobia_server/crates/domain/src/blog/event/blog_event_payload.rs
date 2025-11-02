use appletheia::event::EventPayload;
use serde::{Deserialize, Serialize};

use super::BlogEventPayloadError;

#[derive(Clone, Debug, Eq, PartialEq, Hash, Deserialize, Serialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum BlogEventPayload {}

impl EventPayload for BlogEventPayload {
    type Error = BlogEventPayloadError;
}
