// #![warn(missing_docs)]
// #![warn(clippy::missing_docs_in_private_items)]

use controllers::app::{App, MainMenu};
use models::{
    domain::{
        item::{Category, Item},
        member::Member,
        ToRow,
    },
    system::System,
};
use prettytable::{table, Table};
use views::{
    console::Console,
    main_view::{CliMainView, MainView},
};

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
    );
    // let turing1 = Member::new(
    //     "Turing".to_owned(),
    //     "allan@enigma.com".to_owned(),
    //     "123".to_owned(),
    // );
    // let turing2 = Member::new(
    //     "Turing".to_owned(),
    //     "turing@enigma.com".to_owned(),
    //     "123456".to_owned(),
    // );
    // let turing3 = Member::new(
    //     "Turing".to_owned(),
    //     "turing@enigma.com".to_owned(),
    //     "123".to_owned(),
    // );
    // let mut members = Table::new();

    // members.add_row(allan.to_row());
    // members.add_row(turing1.to_row());
    // members.add_row(turing2.to_row());
    // members.add_row(turing3.to_row());

    let con = Console::new();
    let item = Item::new(
        "Monopoly".to_owned(),
        "A Family Game".to_owned(),
        Category::Game,
        20f64,
    );
    let mut table = Table::new();
    table.add_row(item.to_row());
    con.table(table);
    // con.table(members);
    // let model = System::new();
    // let view = CliMainView::new();
    let app = MainMenu::new();
    // app.run(model, view);
}
