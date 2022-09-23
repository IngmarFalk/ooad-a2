use rand::{distributions::Alphanumeric, thread_rng, Rng};
use shared::{CFromMap, CFromStr, CToMap, CToStr};
use std::collections::HashMap;
use std::ops::Add;
use std::str::FromStr;

use super::domain::FromMap;
use super::system::MError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UuidType {
    Member,
    Item,
    Contract,
    Other,
}

impl std::fmt::Display for UuidType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            UuidType::Member => f.write_str("Member"),
            UuidType::Item => f.write_str("Item"),
            UuidType::Contract => f.write_str("Contract"),
            UuidType::Other => f.write_str("Other"),
        }
    }
}

impl FromStr for UuidType {
    type Err = MError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "member" => Ok(UuidType::Member),
            "item" => Ok(UuidType::Item),
            "contract" => Ok(UuidType::Contract),
            _ => Ok(UuidType::Other),
        }
    }
}

impl Default for UuidType {
    fn default() -> Self {
        UuidType::Member
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, CFromStr, CToStr, CFromMap, CToMap)]
pub struct Uuid {
    pub len: usize,
    uuid_type: UuidType,
    value: String,
}

impl Uuid {
    fn new(uuid_type: UuidType) -> Self {
        let len: usize = 6;
        let value: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(len)
            .map(char::from)
            .collect();
        Uuid {
            value,
            uuid_type,
            ..Default::default()
        }
    }

    fn with_len(len: usize) -> Self {
        let value: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(len)
            .map(char::from)
            .collect();
        Uuid {
            len,
            value,
            ..Default::default()
        }
    }

    pub fn empty() -> Self {
        Uuid {
            len: 0,
            value: "".to_owned(),
            ..Default::default()
        }
    }

    pub fn member() -> Self {
        Uuid::new(UuidType::Member)
    }

    pub fn item() -> Self {
        let item_uuid = Uuid::new(UuidType::Item);
        item_uuid
    }

    pub fn contract() -> Self {
        let contract_uuid = Uuid::new(UuidType::Contract);
        contract_uuid
    }

    fn set_value(&mut self, val: String) {
        self.value = val;
    }

    fn set_len(&mut self, val: usize) {
        self.len = val;
    }

    pub fn gen(len: usize) -> String {
        thread_rng()
            .sample_iter(&Alphanumeric)
            .take(len)
            .map(char::from)
            .collect()
    }
}

impl Add for Uuid {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut out = String::new();
        let mut new_uuid = Uuid::empty();

        out.push_str(&self.value.clone());
        out.push('-');
        out.push_str(&rhs.value.clone());
        new_uuid.set_value(out);
        new_uuid.set_len(new_uuid.value.len());

        new_uuid
    }
}

impl Default for Uuid {
    fn default() -> Self {
        Uuid {
            len: 6,
            uuid_type: UuidType::Member,
            value: Uuid::gen(6),
        }
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
