use models::{item::Item, member::Member};
use views::member_view::MemberDisplay;

use crate::views::member_view::MemberView;

pub mod controllers;
pub mod models;
pub mod views;

// use crate::{controllers::app::App, models::system::System, views::main_view::MainView};

fn main() {
    let turing3 = Member::new(
        "Turing".to_owned(),
        "turing@enigma.com".to_owned(),
        "123".to_owned(),
        500f64,
        vec![Item::new(
            crate::models::item::Category::Game,
            "Monopoly".to_owned(),
            "Family Game".to_owned(),
            None,
            chrono::offset::Local::now(),
            20f64,
        )],
    );
    let view = MemberView::new();
    // view.display_member_simple(turing3.clone());
    // view.display_member_verbose(turing3.clone());
    view.get_str_input("Name: ");
}
