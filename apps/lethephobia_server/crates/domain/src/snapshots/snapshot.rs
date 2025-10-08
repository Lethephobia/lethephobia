use crate::value_objects::{AggregateId, AggregateVersion, CreatedAt, SnapshotId};

pub trait Snapshot: Send + Sync + 'static {
    type AggregateId: AggregateId;

    fn id(&self) -> SnapshotId;

    fn aggregate_type(&self) -> &'static str;

    fn aggregate_version(&self) -> AggregateVersion;

    fn aggregate_id(&self) -> Self::AggregateId;

    fn created_at(&self) -> CreatedAt;
}
