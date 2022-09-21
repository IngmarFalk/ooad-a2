use crate::{
    types::{Controller, Model, View},
    views::main_view::MainView,
};

pub trait App {
    fn run<M, V>(&self, model: M, view: V)
    where
        M: Model,
        V: View + MainView;
}

#[derive(Debug)]
pub struct MainApp;

impl Controller for MainApp {}

impl MainApp {
    pub fn new() -> MainApp {
        MainApp {}
    }
}

// impl App for MainApp {
//     fn run<M, V>(&self, model: M, view: V) {
//         println!("hello world");
//     }
// }
