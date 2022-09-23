// #![warn(missing_docs)]
// #![warn(clippy::missing_docs_in_private_items)]
#![crate_type = "proc-macro"]

use models::domain::{
    item::{Category, Item},
    member::Member,
    Data,
};
use views::{
    console::Console,
    item_view::{CliItemView, ItemView},
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
    let item = Item::default()
        .name("Monopoly".to_owned())
        .description("Family Game".to_owned())
        .category(Category::Game)
        .owner(allan.clone())
        .cost_per_day(20f64);

    // let item_view = CliItemView::new();
    // item_view.display_item_info(item)
    let con = Console::new();
    con.title();
    con.table(allan.to_table());
}
