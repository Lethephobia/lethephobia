use std::fmt::Debug;

pub trait EventPayload: Clone + Debug + Eq + Send + Sync + 'static {
    fn event_type(&self) -> &'static str;
}
