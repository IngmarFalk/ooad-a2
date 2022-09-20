use std::collections::HashMap;

use prettytable::{Row, Table};

use crate::types::StringMap;

pub mod contract;
pub mod item;
pub mod member;

pub trait Data: FromMap + ToMap {
    fn to_row(&self) -> Row;
    fn to_table(&self) -> Table;
    fn head(&self) -> Vec<&str>;
    fn head_allowed_mutable(&self) -> Vec<&str>;
}

pub trait FromMap {
    fn from_partial_map(data: StringMap) -> Self;
    fn from_complete_map(data: StringMap) -> Self;
    fn copy_with(&self, data: StringMap) -> Self;
}

pub trait ToMap {
    fn to_map(&self) -> StringMap;
    fn to_allowed_mutable_map(&self) -> StringMap;
    fn to_buffers_map(&self) -> StringMap;
}
