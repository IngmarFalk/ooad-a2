#[cfg(test)]
mod cdate_tests {
    use crate::models::date_wrapper::DateWrapper;
    use chrono::NaiveDate;

    #[test]
    fn test_lower_than() {
        let date1 = DateWrapper::now();
        let date2 = DateWrapper::new(NaiveDate::from_ymd(2015, 6, 3));
        assert_eq!(date2 < date1, true);
        assert_eq!(date2 <= date1, true);
    }

    #[test]
    fn test_bigger_than() {
        let date1 = DateWrapper::now();
        let date2 = DateWrapper::new(NaiveDate::from_ymd(2015, 6, 3));
        assert_eq!(date1 > date2, true);
        assert_eq!(date1 >= date2, true);
    }

    #[test]
    fn test_eq() {
        let date1 = DateWrapper::new(NaiveDate::from_ymd(2015, 6, 3));
        let date2 = DateWrapper::new(NaiveDate::from_ymd(2015, 6, 3));
        assert_eq!(date1 == date2, true);
    }
}
