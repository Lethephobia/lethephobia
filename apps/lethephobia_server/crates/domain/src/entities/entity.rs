use crate::value_objects::EntityId;

pub trait Entity {
    type Id: EntityId;

    fn id(&self) -> Self::Id;
}
