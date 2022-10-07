use thiserror::Error;

/// The Console.
pub mod console;
/// Contraact view.
pub mod contract_view;
/// Item view.
pub mod item_view;
/// Main view.
pub mod main_view;
/// Member View.
pub mod member_view;
/// Simulator View.
pub mod simulator_view;

/// Trait that allows the console to display any struct that implements this trait.
pub trait Options {
    /// Returns the enums stringified version as well as its current state.
    fn as_tuple(&self) -> (String, Self);
    /// Returns the possible choices of the enum.
    fn options() -> Vec<String>;
    /// Creates the enum from an integer choice.
    fn from_choice(choice: usize) -> Self;
}

#[derive(Debug, Error)]
struct InvalidInput;

impl std::fmt::Display for InvalidInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Invalid Input")
    }
}
