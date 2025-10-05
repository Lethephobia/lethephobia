use crate::entities::Entity;
use crate::errors::DomainError;
use crate::events::Event;
use crate::value_objects::{AggregateId, AggregateVersion};

pub trait Aggregate: Entity
where
    Self::Id: AggregateId,
{
    type Event: Event;
    type Error: DomainError;

    fn id(&self) -> Self::Id;

    fn version(&self) -> AggregateVersion;
    fn set_version(&mut self, version: AggregateVersion);

    fn uncommitted_events(&self) -> &[Self::Event];
    fn uncommitted_events_mut(&mut self) -> &mut Vec<Self::Event>;
    fn record_uncommitted_event(&mut self, event: Self::Event);

    fn handle_event(&mut self, event: &Self::Event) -> Result<(), Self::Error>;

    fn bump_version(&mut self) {
        self.set_version(self.version().next());
    }

    fn take_uncommitted_events(&mut self) -> Vec<Self::Event> {
        let events = self.uncommitted_events_mut().drain(..).collect();
        events
    }

    fn apply_event(&mut self, event: Self::Event) -> Result<(), Self::Error> {
        self.handle_event(&event)?;
        self.record_uncommitted_event(event);
        self.bump_version();
        Ok(())
    }

    fn load_event(&mut self, event: Self::Event) -> Result<(), Self::Error> {
        self.handle_event(&event)?;
        self.bump_version();
        Ok(())
    }
}
