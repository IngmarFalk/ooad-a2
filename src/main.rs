pub mod controllers;
pub mod models;
pub mod views;

use crate::{
    controllers::app::{App, MainApp},
    models::system::System,
    views::main_view::MainView,
};

fn main() {
    let main_view: MainView = MainView {};
    let system: System = System {
        active_contracts: vec![],
        members: vec![],
    };
    let main_app: App = App { system, main_view };
}
