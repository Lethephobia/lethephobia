use std::{fmt, fmt::Display};

use uuid::Uuid;

use crate::errors::EventIdError;
use crate::value_objects::ValueObject;
use crate::value_objects::core::id::Id;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct EventId(Id);

impl EventId {
    pub fn new() -> Self {
        Self(Id::new())
    }

    pub fn value(self) -> Id {
        self.0
    }
}

impl Default for EventId {
    fn default() -> Self {
        Self::new()
    }
}

impl ValueObject for EventId {}

impl TryFrom<Uuid> for EventId {
    type Error = EventIdError;

    fn try_from(value: Uuid) -> Result<Self, Self::Error> {
        Ok(Self(Id::try_from(value)?))
    }
}

impl From<EventId> for Uuid {
    fn from(value: EventId) -> Self {
        value.0.value()
    }
}

impl From<Id> for EventId {
    fn from(value: Id) -> Self {
        Self(value)
    }
}

impl Display for EventId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::errors::IdError;
    use uuid::{Uuid, Version};

    #[test]
    fn new_generates_uuid_v7() {
        let uuid: Uuid = EventId::new().into();

        assert_eq!(uuid.get_version(), Some(Version::SortRand));
    }

    #[test]
    fn default_generates_uuid_v7() {
        let uuid: Uuid = EventId::default().into();

        assert_eq!(uuid.get_version(), Some(Version::SortRand));
    }

    #[test]
    fn try_from_accepts_uuid_v7() {
        let uuid = Uuid::now_v7();
        let event_id = EventId::try_from(uuid).expect("uuidv7 should be accepted");

        assert_eq!(Uuid::from(event_id), uuid);
    }

    #[test]
    fn try_from_rejects_non_uuid_v7() {
        let uuid = Uuid::nil();

        match EventId::try_from(uuid) {
            Err(EventIdError::Id(IdError::NotUuidV7(returned))) => assert_eq!(returned, uuid),
            other => panic!("expected NotUuidV7 error via EventIdError::Id, got {other:?}"),
        }
    }

    #[test]
    fn from_id_preserves_value() {
        let id = Id::new();
        let event_id = EventId::from(id);

        assert_eq!(event_id.value(), id);
    }

    #[test]
    fn display_formats_underlying_uuid() {
        let uuid = Uuid::now_v7();
        let event_id = EventId::try_from(uuid).expect("uuidv7 should be accepted");

        assert_eq!(event_id.to_string(), uuid.to_string());
    }
}
