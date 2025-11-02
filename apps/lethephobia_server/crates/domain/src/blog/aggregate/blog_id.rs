use std::{fmt, fmt::Display};

use appletheia::aggregate::AggregateId;
use serde::{Deserialize, Serialize};
use uuid::{Uuid, Version};

use super::BlogIdError;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd, Deserialize, Serialize)]
#[serde(try_from = "Uuid", into = "Uuid")]
pub struct BlogId(Uuid);

impl BlogId {
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }
}

impl Default for BlogId {
    fn default() -> Self {
        Self::new()
    }
}

impl AggregateId for BlogId {
    type Error = BlogIdError;

    fn value(self) -> Uuid {
        self.0
    }

    fn try_from_uuid(value: Uuid) -> Result<Self, Self::Error> {
        match value.get_version() {
            Some(Version::SortRand) => Ok(Self(value)),
            _ => Err(BlogIdError::NotUuidV7(value)),
        }
    }
}

impl TryFrom<Uuid> for BlogId {
    type Error = BlogIdError;

    fn try_from(value: Uuid) -> Result<Self, Self::Error> {
        Self::try_from_uuid(value)
    }
}

impl From<BlogId> for Uuid {
    fn from(value: BlogId) -> Self {
        value.value()
    }
}

impl Display for BlogId {
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
        let uuid: Uuid = BlogId::new().into();

        assert_eq!(uuid.get_version(), Some(Version::SortRand));
    }

    #[test]
    fn default_generates_uuid_v7() {
        let uuid: Uuid = BlogId::default().into();

        assert_eq!(uuid.get_version(), Some(Version::SortRand));
    }

    #[test]
    fn try_from_accepts_uuid_v7() {
        let uuid = Uuid::now_v7();
        let blog_id = BlogId::try_from(uuid).expect("uuidv7 should be accepted");

        assert_eq!(Uuid::from(blog_id), uuid);
    }

    #[test]
    fn try_from_rejects_non_uuid_v7() {
        let uuid = Uuid::nil();

        match BlogId::try_from(uuid) {
            Err(BlogIdError::NotUuidV7(returned)) => assert_eq!(returned, uuid),
            other => panic!("expected NotUuidV7 error, got {other:?}"),
        }
    }

    #[test]
    fn from_id_preserves_value() {
        let id = Uuid::now_v7();
        let blog_id = BlogId::try_from(id).expect("uuidv7 should be accepted");

        assert_eq!(blog_id.value(), id);
    }
}
