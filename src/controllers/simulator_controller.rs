use crate::{
    models::system::LendingSystem,
    types::{Model, View},
    views::simulator_view::SimulatorView,
};

use super::app::App;

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
    M: Model + LendingSystem,
    V: View + SimulatorView,
{
    pub fn new(model: M, view: V) -> Self {
        Self { model, view }
    }
}

impl<M, V> App for SimulatorController<M, V>
where
    M: Model + LendingSystem,
    V: View + SimulatorView,
{
    fn run(&self) {
        todo!()
    }
}
