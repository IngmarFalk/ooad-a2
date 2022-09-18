use std::fmt;

use rand::{distributions::Alphanumeric, thread_rng, Rng};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
        Uuid { len, value }
    }

    pub fn with_len(len: usize) -> Self {
        let value: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(len)
            .map(char::from)
            .collect();
        Uuid { len, value }
    }
}

impl fmt::Display for Uuid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
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
        let uuid = Uuid::new();
        assert_eq!(uuid.len, 6);

        let uuid2 = Uuid::with_len(15);
        assert_eq!(uuid2.len, 15);
    }
}
