use crate::types::StringMap;
use prettytable::{Row, Table};
use std::{fmt, str::FromStr};

pub mod contract;
pub mod item;
pub mod member;
pub mod system;

pub trait Data: FromMap + ToMap + FromStr + fmt::Display {
    fn to_row(&self) -> Row;
    fn to_table(&self) -> Table;
    fn head() -> Vec<String>;
    fn head_allowed_mutable() -> Vec<String>;
}

pub trait FromMap {
    fn from_complete_map(data: StringMap) -> Self;
    fn copy_with(&self, data: StringMap) -> Self;
}

pub trait ToMap {
    fn to_map(&self) -> StringMap;
    fn to_map_allowed_mutable(&self) -> StringMap;
}
