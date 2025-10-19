use crate::aggregates::AggregateState;
use crate::value_objects::{AggregateVersion, CreatedAt, SnapshotId};

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Snapshot<S: AggregateState> {
    id: SnapshotId,
    aggregate_version: AggregateVersion,
    state: S,
    created_at: CreatedAt,
}

impl<S: AggregateState> Snapshot<S> {
    pub fn new(aggregate_version: AggregateVersion, state: S) -> Self {
        Self {
            id: SnapshotId::new(),
            aggregate_version,
            state,
            created_at: CreatedAt::now(),
        }
    }

    pub fn id(&self) -> SnapshotId {
        self.id
    }

    pub fn aggregate_id(&self) -> S::Id {
        self.state.id()
    }

    pub fn aggregate_version(&self) -> AggregateVersion {
        self.aggregate_version
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
