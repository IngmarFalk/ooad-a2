use crate::types::View;

use super::console::Console;

pub trait SimulatorView {}

pub struct CliSimulatorView {
    console: Console,
}

impl View for CliSimulatorView {}

impl CliSimulatorView {
    pub fn new() -> Self {
        Self { console: Console }
    }
}

impl SimulatorView for CliSimulatorView {}
