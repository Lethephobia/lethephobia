use std::fmt::Debug;

use crate::events::EventPayload;
use crate::value_objects::{AggregateId, AggregateVersion, CreatedAt, EventId};

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Event<A: AggregateId, P: EventPayload> {
    id: EventId,
    aggregate_id: A,
    aggregate_version: AggregateVersion,
    payload: P,
    created_at: CreatedAt,
}

impl<A: AggregateId, P: EventPayload> Event<A, P> {
    pub fn new(aggregate_id: A, aggregate_version: AggregateVersion, payload: P) -> Self {
        Self {
            id: EventId::new(),
            aggregate_id,
            aggregate_version,
            payload,
            created_at: CreatedAt::now(),
        }
    }

    pub fn id(&self) -> EventId {
        self.id
    }

    pub fn aggregate_id(&self) -> A {
        self.aggregate_id
    }

    pub fn aggregate_version(&self) -> AggregateVersion {
        self.aggregate_version
    }

    pub fn payload(&self) -> &P {
        &self.payload
    }

    pub fn created_at(&self) -> CreatedAt {
        self.created_at
    }
}
