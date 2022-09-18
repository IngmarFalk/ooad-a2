use crate::{
    models::system::System,
    types::{Model, View},
    views::{
        console::Console,
        main_view::{CliMainView, MainView},
    },
};

pub trait App {
    fn run<M, V>(&self, model: M, view: V)
    where
        M: Model,
        V: View + MainView;
}

#[derive(Debug)]
pub struct MainMenu;

impl MainMenu {
    pub fn new() -> MainMenu {
        MainMenu {}
    }
}

impl App for MainMenu {
    fn run<M, V>(&self, model: M, view: V) {
        println!("hello world");
    }
}
