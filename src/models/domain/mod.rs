use prettytable::{Row, Table};

pub mod contract;
pub mod item;
pub mod member;

pub trait Data {
    fn to_row(&self) -> Row;
    fn head(&self) -> Row;
    fn to_table(&self) -> Table;
}
