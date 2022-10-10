use super::{
    console::{Console, Ui},
    Options,
};
use crate::types::View;
use shared::COptions;
use std::str::FromStr;

#[derive(Debug, COptions)]
pub enum SimulatorOption {
    /// Increments Day for the system.
    IncrDay,
    /// Goes Back.
    Back,
    /// Quits The Application.
    Quit,
    /// Any Other other choice.
    #[other]
    Other,
}

/// Defines all methods for the simulator view.
pub trait SimulatorView {
    /// Shows all the options for the simulator.
    fn simulator_menu(&self) -> SimulatorOption;
    /// waits for user.
    fn wait(&self, display: &str);
}

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

impl SimulatorView for CliSimulatorView {
    fn simulator_menu(&self) -> SimulatorOption {
        self.console.title();
        let choice = self.console.show_menu(SimulatorOption::options());
        match choice {
            SimulatorOption::Other => self.simulator_menu(),
            _ => choice,
        }
    }

    fn wait(&self, display: &str) {
        self.console.wait(display)
    }
}
