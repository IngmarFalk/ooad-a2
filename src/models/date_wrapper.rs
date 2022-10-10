use chrono::{Duration, ParseError};
use std::str::FromStr;

/// Date wrapper for `chrono::Date`
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct DateWrapper {
    date: chrono::NaiveDate,
}

impl DateWrapper {
    /// Creates a new CDate.
    pub fn new(date: chrono::NaiveDate) -> Self {
        Self { date }
    }

    pub fn now() -> Self {
        Self {
            date: chrono::offset::Local::now().naive_local().date(),
        }
    }

    pub fn in_days(days: i64) -> Self {
        Self {
            date: chrono::offset::Local::now().naive_local().date() + Duration::days(days),
        }
    }

    pub fn add_days(&self, days: i64) -> Self {
        let date = self.as_naive_date();
        DateWrapper::new(date + chrono::Duration::days(days))
    }

    pub fn as_naive_date(&self) -> chrono::NaiveDate {
        self.date
    }

    pub fn days_from(&self, other: &DateWrapper) -> Option<i64> {
        let total = self.date - other.date;
        match total.num_days() {
            i if i > 0 => Some(i),
            _ => None,
        }
    }

    pub fn days_till(&self, other: &DateWrapper) -> Option<i64> {
        let total = other.date - self.date;
        match total.num_days() {
            i if i > 0 => Some(i),
            _ => None,
        }
    }
}

impl std::fmt::Display for DateWrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.date))
    }
}

impl Default for DateWrapper {
    fn default() -> Self {
        Self::now()
    }
}

impl FromStr for DateWrapper {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d") {
            Ok(date) => Ok(DateWrapper { date }),
            Err(err) => Err(err),
        }
    }
}
