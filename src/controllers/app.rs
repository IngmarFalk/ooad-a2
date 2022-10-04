use shared::controller;

use super::{
    item_controller::ItemController, member_controller::MemberController,
    simulator_controller::SimulatorController,
};
use crate::{
    models::domain::system::LendingSystem,
    types::{Model, View},
    views::{
        item_view::CliItemView,
        main_view::{MainMenuOption, MainView},
        member_view::CliMemberView,
        simulator_view::CliSimulatorView,
    },
};

pub trait App<T> {
    fn run(&mut self, sys: T) -> T;
}

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
    pub fn start(&mut self) {
        self.run(self.model.clone());
    }
}

impl<M, V> App<M> for MainApp<M, V>
where
    M: Model + LendingSystem + Clone,
    V: View + MainView,
{
    fn run(&mut self, sys: M) -> M {
        let choice = self.view.main_menu();
        let state = match choice {
            MainMenuOption::MembersPage => {
                let member_view = CliMemberView::new();
                let mut controller = MemberController::new(self.model.clone(), member_view);
                controller.run(sys)
            }
            MainMenuOption::ItemsPage => {
                let item_view = CliItemView::new();
                let mut controller = ItemController::new(self.model.clone(), item_view);
                controller.run(sys)
            }
            MainMenuOption::Simulator => {
                let simulator_view = CliSimulatorView::new();
                let mut controller = SimulatorController::new(self.model.clone(), simulator_view);
                controller.run(sys)
            }
            MainMenuOption::Quit => std::process::exit(0),
            _ => sys,
        };

        self.run(state)
    }
}
