use std::{fmt, fmt::Display};

use appletheia::aggregate::AggregateId;
use appletheia::identifier::Id;
use uuid::Uuid;

use super::BlogIdError;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct BlogId(Id);

impl BlogId {
    pub fn new() -> Self {
        Self(Id::new())
    }
}

impl Default for BlogId {
    fn default() -> Self {
        Self::new()
    }
}

impl AggregateId for BlogId {
    fn value(self) -> Id {
        self.0
    }
}

impl TryFrom<Uuid> for BlogId {
    type Error = BlogIdError;

    fn try_from(value: Uuid) -> Result<Self, Self::Error> {
        Ok(Self(Id::try_from(value)?))
    }
}

impl From<BlogId> for Uuid {
    fn from(value: BlogId) -> Self {
        value.0.value()
    }
}

impl From<Id> for BlogId {
    fn from(value: Id) -> Self {
        Self(value)
    }
}

impl Display for BlogId {
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
            Err(BlogIdError::Id(IdError::NotUuidV7(returned))) => assert_eq!(returned, uuid),
            other => panic!("expected NotUuidV7 error via BlogIdError::Id, got {other:?}"),
        }
    }

    #[test]
    fn from_id_preserves_value() {
        let id = Id::new();
        let blog_id = BlogId::from(id);

        assert_eq!(blog_id.value(), id);
    }

    #[test]
    fn display_formats_underlying_uuid() {
        let uuid = Uuid::now_v7();
        let blog_id = BlogId::try_from(uuid).expect("uuidv7 should be accepted");

        assert_eq!(blog_id.to_string(), uuid.to_string());
    }
}
