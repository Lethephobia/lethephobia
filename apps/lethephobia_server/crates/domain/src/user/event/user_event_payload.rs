use appletheia::event::EventPayload;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum UserEventPayload {}

impl EventPayload for UserEventPayload {
    fn event_type(&self) -> &'static str {
        panic!("unimplemented");
    }
}
