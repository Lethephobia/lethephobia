use crate::aggregate_states::AggregateState;
use crate::errors::AggregateError;
use crate::events::Event;
use crate::snapshots::Snapshot;
use crate::value_objects::AggregateVersion;

pub trait Aggregate {
    type Event: Event;
    type State: AggregateState;
    type Snapshot: Snapshot<State = Self::State>;
    type Error: From<AggregateError>;

    fn state(&self) -> &Option<Self::State>;
    fn state_mut(&mut self) -> &mut Option<Self::State>;

    fn version(&self) -> AggregateVersion;
    fn set_version(&mut self, version: AggregateVersion);

    fn uncommitted_events(&self) -> &[Self::Event];
    fn uncommitted_events_mut(&mut self) -> &mut Vec<Self::Event>;
    fn record_uncommitted_event(&mut self, event: Self::Event);

    fn handle_event(&mut self, event: &Self::Event) -> Result<(), Self::Error>;

    fn bump_version(&mut self) -> Result<(), Self::Error> {
        let next_version = self.version().try_next().map_err(AggregateError::Version)?;
        self.set_version(next_version);
        Ok(())
    }

    fn take_uncommitted_events(&mut self) -> Vec<Self::Event> {
        let events = self.uncommitted_events_mut().drain(..).collect();
        events
    }

    fn validate_next_event(&self, event: &Self::Event) -> Result<(), Self::Error> {
        let next_version = self.version().try_next().map_err(AggregateError::Version)?;
        if event.aggregate_version() != next_version {
            return Err(AggregateError::InvalidNextEventVersion(
                event.aggregate_version(),
                next_version,
            ))?;
        }
        Ok(())
    }

    fn apply_event(&mut self, event: Self::Event) -> Result<(), Self::Error> {
        self.validate_next_event(&event)?;
        self.handle_event(&event)?;
        self.record_uncommitted_event(event);
        self.bump_version()?;
        Ok(())
    }

    fn load_event(&mut self, event: Self::Event) -> Result<(), Self::Error> {
        self.validate_next_event(&event)?;
        self.handle_event(&event)?;
        self.bump_version()?;
        Ok(())
    }

    fn load_snapshot(&mut self, snapshot: Self::Snapshot) -> Result<(), Self::Error> {
        let version = snapshot.aggregate_version();
        let state = snapshot.into_state();
        self.state_mut().replace(state);
        self.set_version(version);
        Ok(())
    }

    fn load_events(
        &mut self,
        events: Vec<Self::Event>,
        snapshot: Option<Self::Snapshot>,
    ) -> Result<(), Self::Error> {
        if let Some(snapshot) = snapshot {
            self.load_snapshot(snapshot)?;
        }
        for event in events {
            self.load_event(event)?;
        }
        Ok(())
    }
}
