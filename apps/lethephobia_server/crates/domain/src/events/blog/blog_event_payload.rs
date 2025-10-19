use crate::events::EventPayload;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum BlogEventPayload {}

impl EventPayload for BlogEventPayload {
    fn event_type(&self) -> &'static str {
        panic!("unimplemented");
    }
}
