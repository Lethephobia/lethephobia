use crate::value_objects::{AggregateId, AggregateVersion, CreatedAt, EventId};

pub trait Event: Send + Sync + 'static {
    type AggregateId: AggregateId;

    const AGGREGATE_TYPE: &'static str;
    const EVENT_TYPE: &'static str;

    fn id(&self) -> EventId;

    fn aggregate_id(&self) -> Self::AggregateId;

    fn aggregate_version(&self) -> AggregateVersion;

    fn created_at(&self) -> CreatedAt;
}
