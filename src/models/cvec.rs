use std::{fmt::Display, str::FromStr};

use super::domain::system::SysError;

/// Wrapper fro `Vec<T>`
#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct CVec<T>
where
    T: Display + PartialEq + Clone + FromStr,
{
    /// Contains all values of the vector.
    pub values: Vec<T>,
    /// keeps track of the length.
    pub len: usize,
}

impl<T> CVec<T>
where
    T: Display + PartialEq + Clone + FromStr,
{
    /// Creates a new Cvec.
    pub fn new() -> CVec<T> {
        CVec {
            values: Vec::new(),
            len: 0,
        }
    }

    /// Pushes a new item onto the cvec.
    pub fn push(&mut self, val: T) {
        self.len += 1;
        self.values.push(val);
    }

    /// Removes an item from the cvec.
    pub fn remove(&mut self, val: T) -> Option<T> {
        match self.index_of(&val) {
            Some(i) => {
                self.len -= 1;
                Some(self.values.remove(i))
            }
            None => None,
        }
    }

    /// Gets index of item.
    pub fn index_of(&self, val: &T) -> Option<usize> {
        self.iter().position(|v| v == val)
    }

    /// Updates item at index.
    pub fn set(&mut self, idx: usize, val: &T) {
        self.values[idx] = val.clone();
    }

    /// Returns the an iterator over the values.
    pub fn iter(&self) -> std::slice::Iter<T> {
        self.values.iter()
    }

    /// Returns the length
    pub fn len(&self) -> usize {
        self.values.len()
    }

    /// Checks if the cvec is empty.
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    /// Returns the values as a vector.
    pub fn to_vec(&self) -> Vec<T> {
        self.values.clone()
    }
}

impl<T> Display for CVec<T>
where
    T: Display + PartialEq + Clone + FromStr,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.values.is_empty() {
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
    type Err = SysError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.eq("-") {
            return Ok(CVec::new());
        }

        let mut arr: Vec<T> = Vec::new();
        let new_str = s.replace(['[', ']'], "");
        for item in new_str.split(';') {
            let temp = T::from_str(item);
            if let Ok(val) = temp {
                arr.push(val)
            }
        }
        let len = arr.len();

        Ok(CVec { values: arr, len })
    }
}
