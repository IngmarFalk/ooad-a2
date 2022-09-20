use crate::types::View;

use super::console::Console;

pub trait MainView {}

pub struct CliMainView {
    console: Console,
}

impl View for CliMainView {}

impl CliMainView {
    pub fn new() -> CliMainView {
        CliMainView {
            console: Console::new(),
        }
    }
}

impl MainView for CliMainView {}
