use super::app::Page;
use crate::{
    models::domain::system::LendingSystem,
    types::{Model, View},
    views::{
        console::{Console, Ui},
        simulator_view::{SimulatorOption, SimulatorView},
    },
};
use shared::controller;

/// The simulator.
#[derive(Debug, Clone)]
#[controller(SimulatorView)]
pub struct SimulatorController<M, V>
where
    M: Model + LendingSystem,
    V: View + SimulatorView,
{
    model: M,
    view: V,
}

impl<M, V> SimulatorController<M, V>
where
    M: Model + LendingSystem + Clone,
    V: View + SimulatorView,
{
    fn incr_day(&mut self) -> M {
        match self.model.incr_time() {
            Ok(_) => {}
            Err(err) => {
                let c = Console::new();
                c.write(err.to_string().as_str());
                c.wait("")
            }
        }
        self.view
            .wait(format!("Day: {}", self.model.now()).as_str());
        self.model.clone()
    }
}

impl<M, V> Page<M> for SimulatorController<M, V>
where
    M: Model + LendingSystem + Clone,
    V: View + SimulatorView,
{
    fn show(&mut self, sys: M) -> M {
        let choice = self.view.simulator_menu();
        let state = match choice {
            SimulatorOption::IncrDay => self.incr_day(),
            SimulatorOption::Back => return sys,
            SimulatorOption::Quit => std::process::exit(0),
            SimulatorOption::Other => sys,
        };
        self.show(state)
    }
}
