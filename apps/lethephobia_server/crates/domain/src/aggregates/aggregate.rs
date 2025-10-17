use std::fmt::Debug;

use crate::aggregate_states::AggregateState;
use crate::errors::AggregateError;
use crate::event_payloads::EventPayload;
use crate::events::Event;
use crate::snapshots::Snapshot;
use crate::value_objects::{AggregateId, AggregateType, AggregateVersion};

pub trait Aggregate: Clone + Debug + Eq + Send + Sync + 'static {
    const TYPE: AggregateType;

    type Id: AggregateId;
    type State: AggregateState<Id = Self::Id>;
    type EventPayload: EventPayload;
    type Error: From<AggregateError>;

    fn state(&self) -> &Option<Self::State>;
    fn state_mut(&mut self) -> &mut Option<Self::State>;

    fn version(&self) -> AggregateVersion;
    fn set_version(&mut self, version: AggregateVersion);

    fn uncommitted_events(&self) -> &[Event<Self::Id, Self::EventPayload>];
    fn uncommitted_events_mut(&mut self) -> &mut Vec<Event<Self::Id, Self::EventPayload>>;
    fn record_uncommitted_event(&mut self, event: Event<Self::Id, Self::EventPayload>);

    fn apply(&mut self, payload: &Self::EventPayload) -> Result<(), Self::Error>;

    fn bump_version(&mut self) -> Result<(), Self::Error> {
        let next_version = self.version().try_next().map_err(AggregateError::Version)?;
        self.set_version(next_version);
        Ok(())
    }

    fn drain_uncommitted_events(&mut self) -> Vec<Event<Self::Id, Self::EventPayload>> {
        self.uncommitted_events_mut().drain(..).collect()
    }

    fn append_event(&mut self, payload: Self::EventPayload) -> Result<(), Self::Error> {
        self.apply(&payload)?;
        self.bump_version()?;
        let aggregate_id = self
            .state()
            .as_ref()
            .map(|state| state.id())
            .ok_or(AggregateError::NoState)?;
        self.record_uncommitted_event(Event::new(
            Self::TYPE,
            aggregate_id,
            self.version(),
            payload,
        ));
        Ok(())
    }

    fn validate_next_event(
        &self,
        event: &Event<Self::Id, Self::EventPayload>,
    ) -> Result<(), Self::Error> {
        let next_version = self.version().try_next().map_err(AggregateError::Version)?;
        if event.aggregate_version() != next_version {
            return Err(AggregateError::InvalidNextEventVersion(
                event.aggregate_version(),
                next_version,
            )
            .into());
        }
        Ok(())
    }

    fn replay_event(
        &mut self,
        event: Event<Self::Id, Self::EventPayload>,
    ) -> Result<(), Self::Error> {
        self.validate_next_event(&event)?;
        self.apply(event.payload())?;
        self.bump_version()?;
        Ok(())
    }

    fn restore_snapshot(
        &mut self,
        snapshot: Snapshot<Self::Id, Self::State>,
    ) -> Result<(), Self::Error> {
        let version = snapshot.aggregate_version();
        let state = snapshot.into_state();
        self.state_mut().replace(state);
        self.set_version(version);
        Ok(())
    }

    fn replay_events<I: IntoIterator<Item = Event<Self::Id, Self::EventPayload>>>(
        &mut self,
        events: I,
        snapshot: Option<Snapshot<Self::Id, Self::State>>,
    ) -> Result<(), Self::Error> {
        if let Some(snapshot) = snapshot {
            self.restore_snapshot(snapshot)?;
        }
        for event in events {
            self.replay_event(event)?;
        }
        Ok(())
    }

    fn to_snapshot(&self) -> Result<Snapshot<Self::Id, Self::State>, Self::Error> {
        self.state()
            .as_ref()
            .map(|state| Snapshot::new(Self::TYPE, state.id(), self.version(), state.clone()))
            .ok_or(AggregateError::NoState.into())
    }
}
