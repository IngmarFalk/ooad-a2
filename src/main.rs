use controllers::app::{App, MainMenu};
use models::{
    domain::{member::Member, ToRow},
    system::System,
};
use prettytable::{table, Table};
use views::main_view::{CliMainView, MainView};

pub mod controllers;
pub mod models;
pub mod types;
pub mod views;

// use crate::{controllers::app::App, models::system::System, views::main_view::MainView};

fn main() {
    let allan = Member::new(
        "Allan".to_owned(),
        "allan@enigma.com".to_owned(),
        "123456".to_owned(),
        500f64,
        vec![],
    );
    let turing1 = Member::new(
        "Turing".to_owned(),
        "allan@enigma.com".to_owned(),
        "123".to_owned(),
        500f64,
        vec![],
    );
    let turing2 = Member::new(
        "Turing".to_owned(),
        "turing@enigma.com".to_owned(),
        "123456".to_owned(),
        500f64,
        vec![],
    );
    let turing3 = Member::new(
        "Turing".to_owned(),
        "turing@enigma.com".to_owned(),
        "123".to_owned(),
        500f64,
        vec![],
    );
    // let mut members = Table::new();

    // members.add_row(allan.to_row());
    // members.add_row(turing1.to_row());
    // members.add_row(turing2.to_row());
    // members.add_row(turing3.to_row());

    // let con = Console::new();
    // con.table(members);
    let model = System::new();
    let view = CliMainView::new();
    let app = MainMenu::new();
    app.run(model, view);
    // view.display_member_simple(turing3.clone());
    // view.display_member_verbose(turing3.clone());
    // view.get_str_input("Name: ");
}
