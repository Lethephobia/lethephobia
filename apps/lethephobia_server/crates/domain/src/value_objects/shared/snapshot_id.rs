use std::{fmt, fmt::Display};

use uuid::Uuid;

use crate::errors::SnapshotIdError;
use crate::value_objects::ValueObject;
use crate::value_objects::shared::id::Id;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct SnapshotId(Id);

impl SnapshotId {
    pub fn new() -> Self {
        Self(Id::new())
    }

    pub fn value(self) -> Id {
        self.0
    }
}

impl Default for SnapshotId {
    fn default() -> Self {
        Self::new()
    }
}

impl ValueObject for SnapshotId {}

impl TryFrom<Uuid> for SnapshotId {
    type Error = SnapshotIdError;

    fn try_from(value: Uuid) -> Result<Self, Self::Error> {
        Ok(Self(Id::try_from(value)?))
    }
}

impl From<SnapshotId> for Uuid {
    fn from(value: SnapshotId) -> Self {
        value.0.value()
    }
}

impl From<Id> for SnapshotId {
    fn from(value: Id) -> Self {
        Self(value)
    }
}

impl Display for SnapshotId {
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
        let uuid: Uuid = SnapshotId::new().into();

        assert_eq!(uuid.get_version(), Some(Version::SortRand));
    }

    #[test]
    fn default_generates_uuid_v7() {
        let uuid: Uuid = SnapshotId::default().into();

        assert_eq!(uuid.get_version(), Some(Version::SortRand));
    }

    #[test]
    fn try_from_accepts_uuid_v7() {
        let uuid = Uuid::now_v7();
        let snapshot_id = SnapshotId::try_from(uuid).expect("uuidv7 should be accepted");

        assert_eq!(Uuid::from(snapshot_id), uuid);
    }

    #[test]
    fn try_from_rejects_non_uuid_v7() {
        let uuid = Uuid::nil();

        match SnapshotId::try_from(uuid) {
            Err(SnapshotIdError::Id(IdError::NotUuidV7(returned))) => assert_eq!(returned, uuid),
            other => panic!("expected NotUuidV7 error via SnapshotIdError::Id, got {other:?}"),
        }
    }

    #[test]
    fn from_id_preserves_value() {
        let id = Id::new();
        let snapshot_id = SnapshotId::from(id);

        assert_eq!(snapshot_id.value(), id);
    }

    #[test]
    fn display_formats_underlying_uuid() {
        let uuid = Uuid::now_v7();
        let snapshot_id = SnapshotId::try_from(uuid).expect("uuidv7 should be accepted");

        assert_eq!(snapshot_id.to_string(), uuid.to_string());
    }
}
