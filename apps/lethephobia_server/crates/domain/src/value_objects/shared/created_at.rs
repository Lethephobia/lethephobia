use std::{fmt, fmt::Display};

use chrono::{DateTime, Utc};

use super::super::ValueObject;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct CreatedAt(DateTime<Utc>);

impl CreatedAt {
    pub fn now() -> Self {
        Self(Utc::now())
    }

    pub fn value(self) -> DateTime<Utc> {
        self.0
    }
}

impl Default for CreatedAt {
    fn default() -> Self {
        Self::now()
    }
}

impl ValueObject for CreatedAt {}

impl From<DateTime<Utc>> for CreatedAt {
    fn from(value: DateTime<Utc>) -> Self {
        Self(value)
    }
}

impl From<CreatedAt> for DateTime<Utc> {
    fn from(value: CreatedAt) -> Self {
        value.0
    }
}

impl Display for CreatedAt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn new_produces_timestamp_close_to_now() {
        let before = Utc::now();
        let created = CreatedAt::now();
        let after = Utc::now();

        let created_at = created.value();
        assert!(
            created_at >= before,
            "expected {created_at} to be after {before}"
        );
        assert!(
            created_at <= after,
            "expected {created_at} to be before {after}"
        );
    }

    #[test]
    fn value_returns_inner_datetime() {
        let timestamp = Utc.with_ymd_and_hms(2024, 1, 2, 3, 4, 5).unwrap();
        let created = CreatedAt::from(timestamp.clone());

        assert_eq!(created.value(), timestamp);
    }

    #[test]
    fn conversions_round_trip() {
        let timestamp = Utc.with_ymd_and_hms(2022, 6, 7, 8, 9, 10).unwrap();
        let created: CreatedAt = timestamp.clone().into();
        let back_into_datetime: DateTime<Utc> = created.into();

        assert_eq!(back_into_datetime, timestamp);
    }

    #[test]
    fn display_matches_inner_datetime() {
        let timestamp = Utc.with_ymd_and_hms(2030, 12, 31, 23, 59, 59).unwrap();
        let created = CreatedAt::from(timestamp.clone());

        assert_eq!(created.to_string(), timestamp.to_string());
    }
}
