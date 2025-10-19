use super::{Id, ValueObject};

pub trait EntityId: Copy + Ord + ValueObject {
    fn value(self) -> Id;
}
