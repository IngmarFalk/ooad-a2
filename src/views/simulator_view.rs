use crate::types::View;

use super::console::Console;

/// The simulators view.
pub trait SimulatorView {}

/// Implementation for simulator view trait.
pub struct CliSimulatorView {
    console: Console,
}

impl View for CliSimulatorView {}

impl CliSimulatorView {
    /// Creates a new Cli simulator view.
    pub fn new() -> Self {
        Self { console: Console }
    }
}

impl SimulatorView for CliSimulatorView {}
