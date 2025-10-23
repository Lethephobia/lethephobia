use domain::blog::BlogEvent;
use domain::user::UserEvent;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum EventUnion {
    User(UserEvent),
    Blog(BlogEvent),
}
