use std::fmt;
use std::fmt::Display;
use std::str::FromStr;

use crate::errors::AggregateTypeError;
use crate::value_objects::ValueObject;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum AggregateType {
    User,
    Blog,
}

impl ValueObject for AggregateType {}

impl Display for AggregateType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::User => write!(f, "user"),
            Self::Blog => write!(f, "blog"),
        }
    }
}

impl FromStr for AggregateType {
    type Err = AggregateTypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "user" => Ok(Self::User),
            "blog" => Ok(Self::Blog),
            _ => Err(AggregateTypeError::FailedToParse(s.to_string())),
        }
    }
}
