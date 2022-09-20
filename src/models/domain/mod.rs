use prettytable::{Row, Table};

pub mod contract;
pub mod item;
pub mod member;

pub trait Data {
    fn to_row(&self) -> Row;
    fn head(&self) -> Row;
    fn head_allowed_mutable(&self) -> Row;
    fn to_table(&self) -> Table;
    // fn to_buffers_map(&self) -> BuffersMap;
    // fn to_user_editable_buffers_map(&self) -> BuffersMap;
}
