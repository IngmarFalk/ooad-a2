// #![warn(missing_docs)]
// #![warn(clippy::missing_docs_in_private_items)]
#![crate_type = "proc-macro"]

use controllers::app::{App, MainApp};
use models::system::System;
use views::main_view::CliMainView;

pub mod controllers;
pub mod models;
pub mod types;
pub mod views;

fn main() {
    let system = System::new();
    let main_view = CliMainView::new();
    let mut app = MainApp::new(system, main_view);
    app.run();
}
