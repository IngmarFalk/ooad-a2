use super::console::Console;
use super::Options;
use crate::types::View;
use crate::views::console::Ui;
use shared::COptions;
use std::str::FromStr;

#[derive(Debug, COptions)]
pub enum MainMenuOption {
    MembersPage,
    ItemsPage,
    Simulator,
    Quit,
    #[other]
    Other,
}

pub trait MainView {
    fn main_menu(&self) -> Option<MainMenuOption>;
}

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

impl MainView for CliMainView {
    fn main_menu(&self) -> Option<MainMenuOption> {
        self.console.title();
        let choice: MainMenuOption = self.console.show_menu(MainMenuOption::options());
        match choice {
            MainMenuOption::Other => self.main_menu(),
            _ => Some(choice),
        }
    }
}
