use super::ValueObject;

pub trait EntityId: Copy + Ord + ValueObject {}
