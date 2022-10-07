use super::console::Console;
use super::Options;
use crate::views::console::Ui;
use shared::{COptions, View};
use std::str::FromStr;

/// All options for the main menu.
#[derive(Debug, COptions)]
pub enum MainMenuOption {
    /// goes to the members page.
    MembersPage,
    /// goes to the items page.
    ItemsPage,
    /// goes to the contracts page.
    ContractsPage,
    /// goes to the simulator
    Simulator,
    /// quites the application.
    Quit,
    #[other]
    /// any other choice made by the user.
    Other,
}

/// All methods needed for a concrete implementation of the main view.
pub trait MainView {
    /// Shows all the options for the main menu.
    fn main_menu(&self) -> MainMenuOption;
}

/// A concrete implementation of the main view.
#[derive(View)]
pub struct CliMainView {
    console: Console,
}

impl MainView for CliMainView {
    fn main_menu(&self) -> MainMenuOption {
        self.console.title();
        let choice: MainMenuOption = self.console.show_menu(MainMenuOption::options());
        match choice {
            MainMenuOption::Other => self.main_menu(),
            _ => choice,
        }
    }
}
