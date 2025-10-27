use std::{fmt, fmt::Display};

use uuid::Uuid;

use appletheia::aggregate::AggregateId;
use appletheia::identifier::Id;

use super::UserIdError;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct UserId(Id);

impl UserId {
    pub fn new() -> Self {
        Self(Id::new())
    }
}

impl Default for UserId {
    fn default() -> Self {
        Self::new()
    }
}

impl AggregateId for UserId {
    fn value(self) -> Id {
        self.0
    }
}

impl TryFrom<Uuid> for UserId {
    type Error = UserIdError;

    fn try_from(value: Uuid) -> Result<Self, Self::Error> {
        Ok(Self(Id::try_from(value)?))
    }
}

impl From<UserId> for Uuid {
    fn from(value: UserId) -> Self {
        value.0.value()
    }
}

impl From<Id> for UserId {
    fn from(value: Id) -> Self {
        Self(value)
    }
}

impl Display for UserId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use appletheia::identifier::IdError;
    use uuid::{Uuid, Version};

    #[test]
    fn new_generates_uuid_v7() {
        let uuid: Uuid = UserId::new().into();

        assert_eq!(uuid.get_version(), Some(Version::SortRand));
    }

    #[test]
    fn default_generates_uuid_v7() {
        let uuid: Uuid = UserId::default().into();

        assert_eq!(uuid.get_version(), Some(Version::SortRand));
    }

    #[test]
    fn try_from_accepts_uuid_v7() {
        let uuid = Uuid::now_v7();
        let user_id = UserId::try_from(uuid).expect("uuidv7 should be accepted");

        assert_eq!(Uuid::from(user_id), uuid);
    }

    #[test]
    fn try_from_rejects_non_uuid_v7() {
        let uuid = Uuid::nil();

        match UserId::try_from(uuid) {
            Err(UserIdError::Id(IdError::NotUuidV7(returned))) => assert_eq!(returned, uuid),
            other => panic!("expected NotUuidV7 error via UserIdError::Id, got {other:?}"),
        }
    }

    #[test]
    fn from_id_preserves_value() {
        let id = Id::new();
        let user_id = UserId::from(id);

        assert_eq!(user_id.value(), id);
    }

    #[test]
    fn display_formats_underlying_uuid() {
        let uuid = Uuid::now_v7();
        let user_id = UserId::try_from(uuid).expect("uuidv7 should be accepted");

        assert_eq!(user_id.to_string(), uuid.to_string());
    }
}
