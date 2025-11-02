use std::{fmt, fmt::Display};

use serde::{Deserialize, Serialize};
use uuid::{Uuid, Version};

use appletheia::aggregate::AggregateId;

use super::UserIdError;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd, Deserialize, Serialize)]
#[serde(try_from = "Uuid", into = "Uuid")]
pub struct UserId(Uuid);

impl UserId {
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }
}

impl Default for UserId {
    fn default() -> Self {
        Self::new()
    }
}

impl AggregateId for UserId {
    type Error = UserIdError;

    fn value(self) -> Uuid {
        self.0
    }

    fn try_from_uuid(value: Uuid) -> Result<Self, Self::Error> {
        match value.get_version() {
            Some(Version::SortRand) => Ok(Self(value)),
            _ => Err(UserIdError::NotUuidV7(value)),
        }
    }
}

impl TryFrom<Uuid> for UserId {
    type Error = UserIdError;

    fn try_from(value: Uuid) -> Result<Self, Self::Error> {
        Self::try_from_uuid(value)
    }
}

impl From<UserId> for Uuid {
    fn from(value: UserId) -> Self {
        value.0
    }
}

impl Display for UserId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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
            Err(UserIdError::NotUuidV7(returned)) => assert_eq!(returned, uuid),
            other => panic!("expected NotUuidV7 error, got {other:?}"),
        }
    }
}
