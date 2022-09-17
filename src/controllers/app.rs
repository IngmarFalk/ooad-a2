use crate::{models::system::System, views::main_view::MainView};

pub trait MainApp {
    fn run();
}

pub struct App {
    pub system: System,
    pub main_view: MainView,
}

impl MainApp for App {
    fn run() {}
}

impl App {
    pub fn start() {}
}
