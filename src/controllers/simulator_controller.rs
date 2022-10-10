use super::app::Page;
use crate::{
    models::domain::system::LendingSystem,
    types::{Model, View},
    views::simulator_view::SimulatorView,
};
use shared::controller;

/// The simulator.
#[controller(SimulatorView)]
pub struct SimulatorController<M, V>
where
    M: Model + LendingSystem,
    V: View + SimulatorView,
{
    model: M,
    view: V,
}

impl<M, V> Page<M> for SimulatorController<M, V>
where
    M: Model + LendingSystem,
    V: View + SimulatorView,
{
    fn show(&mut self, sys: M) -> M {
        todo!()
    }
}
