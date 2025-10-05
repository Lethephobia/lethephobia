use std::{fmt, fmt::Display};

use crate::errors::AggregateVersionError;

use super::super::ValueObject;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct AggregateVersion(i64);

impl AggregateVersion {
    pub fn new() -> Self {
        Self(0)
    }

    pub fn value(self) -> i64 {
        self.0
    }

    pub fn next(self) -> Self {
        Self(self.value() + 1)
    }

    pub fn checked_next(self) -> Option<Self> {
        self.value().checked_add(1).map(|v| Self(v))
    }

    pub fn try_next(self) -> Result<Self, AggregateVersionError> {
        self.value()
            .checked_add(1)
            .map(|v| Self(v))
            .ok_or(AggregateVersionError::Overflow)
    }
}

impl ValueObject for AggregateVersion {}

impl TryFrom<i64> for AggregateVersion {
    type Error = AggregateVersionError;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        if value < 0 {
            Err(AggregateVersionError::NegativeValue(value))
        } else {
            Ok(Self(value))
        }
    }
}

impl From<AggregateVersion> for i64 {
    fn from(value: AggregateVersion) -> Self {
        value.value()
    }
}

impl Display for AggregateVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_starts_at_zero() {
        let version = AggregateVersion::new();
        assert_eq!(version.value(), 0);
    }

    #[test]
    fn try_from_accepts_non_negative_values() {
        let version = AggregateVersion::try_from(42).expect("expected valid version");
        assert_eq!(version.value(), 42);
    }

    #[test]
    fn try_from_rejects_negative_values() {
        let err = AggregateVersion::try_from(-1).expect_err("expected negative value error");
        match err {
            AggregateVersionError::NegativeValue(v) => assert_eq!(v, -1),
            _ => panic!("unexpected error variant"),
        }
    }

    #[test]
    fn next_increments_version() {
        let current = AggregateVersion::new();
        let next = current.next();
        assert_eq!(next.value(), 1);
    }

    #[test]
    fn checked_next_handles_overflow() {
        let near_max = AggregateVersion::try_from(i64::MAX - 1).unwrap();
        let next = near_max.checked_next().expect("should provide next value");
        assert_eq!(next.value(), i64::MAX);

        let max = AggregateVersion::try_from(i64::MAX).unwrap();
        assert!(max.checked_next().is_none());
    }

    #[test]
    fn try_next_returns_error_on_overflow() {
        let near_max = AggregateVersion::try_from(i64::MAX - 1).unwrap();
        let next = near_max.try_next().expect("should provide next value");
        assert_eq!(next.value(), i64::MAX);

        let max = AggregateVersion::try_from(i64::MAX).unwrap();
        let err = max.try_next().expect_err("expected overflow error");
        match err {
            AggregateVersionError::Overflow => {}
            _ => panic!("unexpected error variant"),
        }
    }

    #[test]
    fn conversions_round_trip() {
        let version = AggregateVersion::try_from(7).unwrap();
        let as_i64: i64 = version.into();
        assert_eq!(as_i64, 7);
    }

    #[test]
    fn display_formats_value() {
        let version = AggregateVersion::try_from(5).unwrap();
        assert_eq!(version.to_string(), "5");
    }
}
