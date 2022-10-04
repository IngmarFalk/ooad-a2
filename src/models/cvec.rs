use std::{fmt::Display, str::FromStr};

use super::domain::system::MError;

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct CVec<T>
where
    T: Display + PartialEq + Clone + FromStr,
{
    pub values: Vec<T>,
    pub len: usize,
}

impl<T> CVec<T>
where
    T: Display + PartialEq + Clone + FromStr,
{
    pub fn new() -> CVec<T> {
        CVec {
            values: Vec::new(),
            len: 0,
        }
    }

    pub fn push(&mut self, val: T) {
        self.values.push(val);
    }

    pub fn remove(&mut self, val: T) -> Option<T> {
        match self.index_of(&val) {
            Some(i) => Some(self.values.remove(i)),
            None => None,
        }
    }

    pub fn index_of(&self, val: &T) -> Option<usize> {
        self.iter().position(|v| v == val)
    }

    pub fn set(&mut self, idx: usize, val: &T) {
        self.values[idx] = val.clone();
    }

    pub fn iter(&self) -> std::slice::Iter<T> {
        self.values.iter()
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn to_vec(&self) -> Vec<T> {
        self.values.clone()
    }
}

impl<T> Display for CVec<T>
where
    T: Display + PartialEq + Clone + FromStr,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.values.len() == 0 {
            return match f.write_str("-") {
                Ok(out) => Ok(out),
                Err(err) => Err(err),
            };
        }
        self.values.iter().fold(Ok(()), |result, val| {
            result.and_then(|_| writeln!(f, "{},", val))
        })
    }
}

impl<T> FromStr for CVec<T>
where
    T: Display + PartialEq + FromStr + Clone,
{
    type Err = MError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.eq("-") {
            return Ok(CVec::new());
        }

        let mut arr: Vec<T> = Vec::new();
        let new_str = s.replace("[", "").replace("]", "");
        for item in new_str.split(";").into_iter() {
            let temp = T::from_str(item);
            match temp {
                Ok(val) => arr.push(val),
                Err(_) => {}
            }
        }
        let len = arr.len();

        Ok(CVec { values: arr, len })
    }
}
