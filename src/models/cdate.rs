use chrono::ParseError;
use std::str::FromStr;

/// Date wrapper for `chrono::Date`
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct CDate {
    date: chrono::NaiveDate,
}

impl CDate {
    /// Creates a new CDate.
    pub fn new() -> Self {
        Self {
            date: chrono::offset::Local::now().naive_local().date(),
        }
    }
}

impl std::fmt::Display for CDate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.date))
    }
}

impl Default for CDate {
    fn default() -> Self {
        CDate::new()
    }
}

impl FromStr for CDate {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d") {
            Ok(date) => Ok(CDate { date }),
            Err(err) => Err(err),
        }
    }
}

#[cfg(test)]
mod cdate_tests {
    use chrono::NaiveDate;

    use super::CDate;

    #[test]
    fn test_lower_than() {
        let date1 = CDate::new();
        let date2 = CDate {
            date: NaiveDate::from_ymd(2015, 6, 3),
        };
        assert_eq!(date2 < date1, true);
        assert_eq!(date2 <= date1, true);
    }

    #[test]
    fn test_bigger_than() {
        let date1 = CDate::new();
        let date2 = CDate {
            date: NaiveDate::from_ymd(2015, 6, 3),
        };
        assert_eq!(date1 > date2, true);
        assert_eq!(date1 >= date2, true);
    }

    #[test]
    fn test_eq() {
        let date1 = CDate {
            date: NaiveDate::from_ymd(2015, 6, 3),
        };
        let date2 = CDate {
            date: NaiveDate::from_ymd(2015, 6, 3),
        };
        assert_eq!(date1 == date2, true);
    }
}
