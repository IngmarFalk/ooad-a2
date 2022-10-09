use prettytable::{Row, Table};
use std::{collections::HashMap, fmt, str::FromStr};

/// Contract model.
pub mod contract;
/// Item model.
pub mod item;
/// Member model.
pub mod member;
/// System model.
pub mod system;

/// Data trait. This trait is required to be implemented by all models that are to be used by the console.
pub trait Data: FromMap + ToMap + FromStr + fmt::Display {
    /// Returns a `prettytable::Row`
    fn to_row(&self) -> Row;
    /// Returns a `prettytable::Table`
    fn to_table(&self) -> Table;
    /// Returns the table titles/head.
    fn head() -> Vec<String>;
    /// Returnes the table titles/head that the user is allowed to set/edit.
    fn head_allowed_mutable() -> Vec<String>;
}

/// For constructing a struct from a hashmap.
pub trait FromMap {
    /// Crom a complete map. This map includes all attributes.
    fn from_complete_map(data: HashMap<String, String>) -> Self;
    /// data map includes (possibly) only some of the attribtes.
    /// The method returns a struct with the same values for each attribute unless
    /// there is a corresponding value in the map.
    fn copy_with_map(&self, data: HashMap<String, String>) -> Self;
}

/// To deconstructing struct into a hashmap.
pub trait ToMap {
    /// To a full map with all attributes.
    fn to_map(&self) -> HashMap<String, String>;
    /// Only the attributes that are allowed to be edited by the user.
    fn to_map_allowed_mutable(&self) -> HashMap<String, String>;
}
