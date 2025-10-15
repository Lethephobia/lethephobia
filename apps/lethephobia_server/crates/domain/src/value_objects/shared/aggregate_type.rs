use std::fmt;
use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum AggregateType {
    User,
    Blog,
}

impl Display for AggregateType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::User => "user",
            Self::Blog => "blog",
        };
        f.write_str(s)
    }
}

impl FromStr for AggregateType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "user" => Ok(Self::User),
            "blog" => Ok(Self::Blog),
            _ => Err(()),
        }
    }
}
