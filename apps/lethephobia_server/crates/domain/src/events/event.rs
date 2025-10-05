use crate::value_objects::{AggregateId, AggregateVersion, CreatedAt, EventId};

pub trait Event: Send + Sync + 'static {
    type AggregateId: AggregateId;

    fn id(&self) -> EventId;

    fn event_type(&self) -> &'static str;

    fn aggregate_type(&self) -> &'static str;

    fn aggregate_id(&self) -> Self::AggregateId;

    fn aggregate_version(&self) -> AggregateVersion;

    fn created_at(&self) -> CreatedAt;
}
