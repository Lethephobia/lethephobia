use crate::aggregate_states::AggregateState;
use crate::value_objects::{AggregateId, AggregateVersion, CreatedAt, SnapshotId};

pub trait Snapshot: Send + Sync + 'static {
    type AggregateId: AggregateId;
    type State: AggregateState;

    const AGGREGATE_TYPE: &'static str;

    fn id(&self) -> SnapshotId;

    fn aggregate_version(&self) -> AggregateVersion;

    fn aggregate_id(&self) -> &Self::AggregateId;

    fn state(&self) -> &Self::State;

    fn into_state(self) -> Self::State;

    fn created_at(&self) -> CreatedAt;
}
