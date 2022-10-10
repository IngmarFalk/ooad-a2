use crate::types::FromMap;
use derive_getters::Getters;
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use shared::{DeriveFromMap, DeriveFromStr, DeriveToMap, DeriveToStr};
use std::collections::HashMap;
use std::str::FromStr;

/// Uuid struct.
///
/// This struct is used to create a unique key for different objects.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    DeriveFromStr,
    DeriveToStr,
    DeriveFromMap,
    DeriveToMap,
    Getters,
)]
pub struct Uuid {
    #[getter(rename = "get_len")]
    len: usize,
    #[getter(rename = "get_value")]
    value: String,
}

impl Uuid {
    /// Creates a new uuid.
    pub fn new() -> Self {
        let len: usize = 6;
        let value: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(len)
            .map(char::from)
            .collect();
        Uuid { value, len }
    }

    /// Creates a new uuid with a specific length.
    pub fn with_len(len: usize) -> Self {
        let value: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(len)
            .map(char::from)
            .collect();
        Uuid { value, len }
    }

    /// Creates a new uuid with no value.
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
