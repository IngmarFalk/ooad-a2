use super::console::Console;
use super::Options;
use crate::views::console::Ui;
use shared::{COptions, View};
use std::str::FromStr;

#[derive(Debug, COptions)]
pub enum MainMenuOption {
    MembersPage,
    ItemsPage,
    ContractsPage,
    Simulator,
    Quit,
    #[other]
    Other,
}

pub trait MainView {
    fn main_menu(&self) -> MainMenuOption;
}

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
