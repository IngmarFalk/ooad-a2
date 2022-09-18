pub mod contract;
pub mod item;
pub mod member;

pub trait ToRow {
    fn to_row(&self) -> Vec<String>;
}
