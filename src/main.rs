//! This is the entry point for the lending app.
// #![deny(missing_docs)]
#![crate_type = "proc-macro"]

use controllers::app::MainApp;
use models::domain::system::{Demo, System};
use views::main_view::CliMainView;

/// Controllers Module.
///
/// Contains workflow and combines views and models functionality.
pub mod controllers;

/// Models Module.
///
/// Contains the data and functionality structs.
pub mod models;

/// Test Module.
///
/// Contains test files for each major model.
pub mod tests;

/// Types.
///
/// Contains Shared types.
pub mod types;

/// Error Module
///
/// Contains errors needed in the system.
pub mod errors;
/// Views Module
///
/// Contains all ui/ux related functionality.
pub mod views;

/// Main method
fn main() {
    let mut system = System::new();
    system.init_demo();
    let main_view = CliMainView::new();
    let mut app = MainApp::new(system, main_view);
    app.start()
}
