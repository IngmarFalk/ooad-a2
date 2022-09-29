use crate::models::system::LendingSystem;
use thiserror::Error;

pub mod console;
pub mod contract_view;
pub mod item_view;
pub mod main_view;
pub mod member_view;
pub mod simulator_view;

pub trait Options {
    fn as_tuple(&self) -> (String, Self);
    fn options() -> Vec<String>;
    fn from_choice(choice: usize) -> Self;
}

#[derive(Debug, Error)]
struct InvalidInput;

impl std::fmt::Display for InvalidInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Invalid Input")
    }
}

pub trait Show {
    fn show_simple(&self, model: &str, system: impl LendingSystem);
    fn show_verbose(&self, model: &str, system: impl LendingSystem);
}
