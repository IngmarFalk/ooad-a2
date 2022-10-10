use shared::controller;

use super::{
    contract_controller::ContractController, item_controller::ItemController,
    member_controller::MemberController, simulator_controller::SimulatorController,
};
use crate::{
    models::domain::system::LendingSystem,
    types::{Model, View},
    views::{
        contract_view::CliContractView,
        item_view::CliItemView,
        main_view::{MainMenuOption, MainView},
        member_view::CliMemberView,
        simulator_view::CliSimulatorView,
    },
};

/// Defines a controller struct.
pub trait Page<T> {
    /// Runs the application/subapplication.
    fn show(&mut self, sys: T) -> T;
}

/// This is the main app of the program.
#[derive(Debug, Clone)]
#[controller(MainView)]
pub struct MainApp<M, V>
where
    M: Model + LendingSystem,
    V: View + MainView,
{
    model: M,
    view: V,
}

impl<M, V> MainApp<M, V>
where
    M: Model + LendingSystem + Clone,
    V: View + MainView,
{
    /// Starts the entire application.
    pub fn start(&mut self) {
        self.show(self.model.clone());
    }
}

impl<M, V> Page<M> for MainApp<M, V>
where
    M: Model + LendingSystem + Clone,
    V: View + MainView,
{
    fn show(&mut self, sys: M) -> M {
        let choice = self.view.main_menu();
        let state = match choice {
            MainMenuOption::MembersPage => {
                let member_view = CliMemberView::new();
                let mut controller = MemberController::new(sys.clone(), member_view);
                controller.show(sys)
            }
            MainMenuOption::ItemsPage => {
                let item_view = CliItemView::new();
                let mut controller = ItemController::new(sys.clone(), item_view);
                controller.show(sys)
            }
            MainMenuOption::Simulator => {
                let simulator_view = CliSimulatorView::new();
                let mut controller = SimulatorController::new(sys.clone(), simulator_view);
                controller.show(sys)
            }
            MainMenuOption::ContractsPage => {
                let contract_view = CliContractView::new();
                let mut controller = ContractController::new(sys.clone(), contract_view);
                controller.show(sys)
            }
            MainMenuOption::Quit => std::process::exit(0),
            _ => sys,
        };

        self.show(state)
    }
}
