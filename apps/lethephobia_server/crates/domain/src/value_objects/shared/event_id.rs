use std::{fmt, fmt::Display};

use uuid::Uuid;

use super::super::ValueObject;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct EventId(Uuid);

impl EventId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn value(self) -> Uuid {
        self.0
    }
}

impl ValueObject for EventId {}

impl From<Uuid> for EventId {
    fn from(value: Uuid) -> Self {
        Self(value)
    }
}

impl From<EventId> for Uuid {
    fn from(value: EventId) -> Self {
        value.value()
    }
}

impl Display for EventId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_produces_non_nil_uuid() {
        let event_id = EventId::new();
        assert_ne!(event_id.value(), Uuid::nil());
    }

    #[test]
    fn value_returns_inner_uuid() {
        let uuid = Uuid::from_u128(0x12345678123456781234567812345678);
        let event_id = EventId::from(uuid);
        assert_eq!(event_id.value(), uuid);
    }

    #[test]
    fn conversions_are_consistent() {
        let uuid = Uuid::from_u128(0x87654321876543218765432187654321);
        let event_id: EventId = uuid.into();
        let back_into_uuid: Uuid = event_id.into();

        assert_eq!(back_into_uuid, uuid);
    }

    #[test]
    fn display_uses_uuid_string() {
        let uuid = Uuid::from_u128(0xabcdefabcdefabcdefabcdefabcdefab);
        let event_id = EventId::from(uuid);

        assert_eq!(event_id.to_string(), uuid.to_string());
    }
}
