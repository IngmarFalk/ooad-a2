use super::domain::FromMap;
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use shared::{CFromMap, CFromStr, CToMap, CToStr};
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq, Hash, CFromStr, CToStr, CFromMap, CToMap)]
pub struct Uuid {
    pub len: usize,
    value: String,
}

impl Uuid {
    pub fn new() -> Self {
        let len: usize = 6;
        let value: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(len)
            .map(char::from)
            .collect();
        Uuid { value, len }
    }

    pub fn with_len(len: usize) -> Self {
        let value: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(len)
            .map(char::from)
            .collect();
        Uuid { value, len }
    }

    pub fn empty() -> Self {
        Uuid {
            len: 0,
            value: "".to_owned(),
        }
    }
}

impl Default for Uuid {
    fn default() -> Self {
        Uuid::new()
    }
}

#[cfg(test)]
mod uuid_test {
    use super::Uuid;

    #[test]
    fn test_len() {
        let uuid = Uuid::default();
        assert_eq!(uuid.len, 6);

        let uuid2 = Uuid::with_len(15);
        assert_eq!(uuid2.len, 15);
    }
}
