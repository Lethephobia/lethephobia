use crate::aggregates::AggregateState;
use crate::value_objects::{AggregateId, AggregateType, AggregateVersion, CreatedAt, SnapshotId};

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Snapshot<A: AggregateId, S: AggregateState<Id = A>> {
    id: SnapshotId,
    aggregate_type: AggregateType,
    aggregate_id: A,
    aggregate_version: AggregateVersion,
    state: S,
    created_at: CreatedAt,
}

impl<A: AggregateId, S: AggregateState<Id = A>> Snapshot<A, S> {
    pub fn new(
        aggregate_type: AggregateType,
        aggregate_id: A,
        aggregate_version: AggregateVersion,
        state: S,
    ) -> Self {
        Self {
            id: SnapshotId::new(),
            aggregate_type,
            aggregate_id,
            aggregate_version,
            state,
            created_at: CreatedAt::now(),
        }
    }

    pub fn id(&self) -> SnapshotId {
        self.id
    }

    pub fn aggregate_type(&self) -> AggregateType {
        self.aggregate_type
    }

    pub fn aggregate_version(&self) -> AggregateVersion {
        self.aggregate_version
    }

    pub fn aggregate_id(&self) -> A {
        self.aggregate_id
    }

    pub fn state(&self) -> &S {
        &self.state
    }

    pub fn into_state(self) -> S {
        self.state
    }

    pub fn created_at(&self) -> CreatedAt {
        self.created_at
    }
}
